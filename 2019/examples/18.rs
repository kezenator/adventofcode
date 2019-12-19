use aoc2019::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::BTreeSet;
use std::collections::HashMap;

#[derive(Debug)]
struct Map
{
    chars: Vec<char>,
    width: i64,
    height: i64,
    all_keys: BTreeSet<char>,
    initial_pos: Vec<Point>,
}

impl Map
{
    fn new(input: &str) -> Self
    {
        let chars = input.chars().collect::<Vec<char>>();
        let width = input.split("\n").nth(0).unwrap().chars().count() as i64;
        let height = (input.chars().count() as i64) / (width + 1);
        let all_keys = input.chars().filter(|&c| c >= 'a' && c <= 'z').collect();
        let initial_pos = (0..((width+1)*height)).filter(|&i| chars[i as usize] == '@').map(|i| Point::new(i % (width + 1), i / (width + 1))).collect();

        Map { chars, width, height, all_keys, initial_pos }
    }

    fn char_at(&self, point: &Point) -> char
    {
        if point.x < 0 || point.y < 0 || point.x >= self.width || point.y >= self.height
        {
            '#'
        }
        else
        {
            self.chars[(point.x + point.y * (self.width + 1)) as usize]
        }
    }

    fn set_char_at(&mut self, point: &Point, ch: char)
    {
        self.chars[(point.x + point.y * (self.width + 1)) as usize] = ch;
    }

    fn neighbours(&self, point: &Point) -> Vec<(Point, char)>
    {
        vec![Point::new(-1, 0), Point::new(1, 0), Point::new(0, -1), Point::new(0, 1)].drain(..)
            .map(|p| point.add(&p))
            .filter(|p| self.char_at(p) != '#')
            .map(|p| (p.clone(), self.char_at(&p)))
            .collect::<Vec<_>>()
    }

    fn pos_of_key(&self, key: char) -> Point
    {
        for y in 0..self.height
        {
            for x in 0..self.width
            {
                if self.chars[(x + y * (self.width + 1)) as usize] == key
                {
                    return Point::new(x, y);
                }
            }
        }
        assert!(false);
        unreachable!();
    }
}

#[derive(Debug, Clone)]
struct State
{
    map: Rc<Map>,
    pos: Vec<Point>,
    keys: BTreeSet<char>,
    opened_doors: BTreeSet<char>,
    ignore_doors: bool,
    ntk: Rc<RefCell<HashMap<State, Vec<(State, usize)>>>>,
}

impl PartialEq for State
{
    fn eq(&self, other: &State) -> bool
    {
        self.pos.eq(&other.pos) && self.keys.eq(&other.keys)
    }
}

impl Eq for State
{
}

impl std::hash::Hash for State
{
    fn hash<H: std::hash::Hasher>(&self, h: &mut H)
    {
        self.pos.hash(h);
        self.keys.hash(h);
    }
}

impl State
{
    fn initial(input: &str) -> Self
    {
        let map = Rc::new(Map::new(input));
        let pos = map.initial_pos.clone();
        let keys = BTreeSet::new();
        let opened_doors = BTreeSet::new();
        let ignore_doors = false;
        let ntk = Rc::new(RefCell::new(HashMap::new()));

        State { map, pos, keys, opened_doors, ignore_doors, ntk }
    }

    fn neighbours(&self, i: usize) -> Vec<State>
    {
        let mut result = Vec::new();

        for (i_pos, ch) in self.map.neighbours(&self.pos[i])
        {
            let mut pos = self.pos.clone();
            pos[i] = i_pos;

            if ch >= 'A' && ch <= 'Z'
            {
                // Can only move here if we have the key,
                // or if we're in the special "ignore_doors" mode
                if self.ignore_doors || self.keys.contains(&ch.to_lowercase().nth(0).unwrap())
                {
                    let mut new_doors = self.opened_doors.clone();
                    new_doors.insert(ch);

                    result.push(State{
                        map: self.map.clone(),
                        pos: pos,
                        keys: self.keys.clone(),
                        opened_doors: new_doors,
                        ignore_doors: self.ignore_doors,
                        ntk: self.ntk.clone(),
                    });
                }
            }
            else if ch >= 'a' && ch <= 'z'
            {
                // Take the key
                let mut new_keys = self.keys.clone();
                new_keys.insert(ch);

                result.push(State{
                    map: self.map.clone(),
                    pos: pos,
                    keys: new_keys,
                    opened_doors: self.opened_doors.clone(),
                    ignore_doors: self.ignore_doors,
                    ntk: self.ntk.clone(),
                });
            }
            else
            {
                // Just move here
                result.push(State{
                    map: self.map.clone(),
                    pos: pos,
                    keys: self.keys.clone(),
                    opened_doors: self.opened_doors.clone(),
                    ignore_doors: self.ignore_doors,
                    ntk: self.ntk.clone(),
                });
            }
        }
        result
    }

    fn next_taken_keys(&self) -> Vec<(State, usize)>
    {
        if let Some(result) = self.ntk.borrow().get(self)
        {
            return result.clone();
        }

        let mut result = Vec::new();

        for &ch in self.map.all_keys.iter()
        {
            if !self.keys.contains(&ch)
            {
                let taken_none_but_ch = |s: &State|
                {
                    for &och in s.keys.iter()
                    {
                        if och != ch && !self.keys.contains(&och)
                        {
                            return false;
                        }
                    }
                    true
                };

                let pos_of_ch = self.map.pos_of_key(ch);

                for i in 0..self.pos.len()
                {
                    if let Some((mut path, cost)) = pathfinding::directed::astar::astar(
                        self,
                        |s| s.neighbours(i).iter().filter(|s| taken_none_but_ch(s)).map(|n| (n.clone(), 1)).collect::<Vec<(State, usize)>>(),
                        |s| s.pos[i].manhatten_dist_to(&pos_of_ch) as usize,
                        |s| s.keys.contains(&ch))
                    {
                        result.push((path.drain(..).last().unwrap(), cost));
                        break;
                    }
                }
            }
        }
        /*println!("=============== NEXT KEYS ======================");
        self.display();
        for s in result.iter()
        {
            println!("COST={}", s.1);
            s.0.display();
        }*/
        println!("Found {} of {}", self.keys.len() + 1, self.map.all_keys.len());

        self.ntk.borrow_mut().insert(self.clone(), result.clone());

        result
    }

    fn has_got_all_keys(&self) -> bool
    {
        self.keys == self.map.all_keys
    }

    fn display(&self)
    {
        let mut paints = Vec::new();
        for y in 0..self.map.height
        {
            for x in 0..self.map.width
            {
                let mut ch = self.map.char_at(&Point::new(x, y));
                if ch >= 'A' && ch <= 'Z'
                {
                    if self.opened_doors.contains(&ch)
                    {
                        ch = '.';
                    }
                }
                else if ch >= 'a' && ch <= 'z'
                {
                    if self.keys.contains(&ch)
                    {
                        ch = '.';
                    }
                }
                else if ch == '@'
                {
                    ch = '.';
                }
                paints.push(PaintPoint::new(Point::new(x, y), Some(ch)));
            }
        }
        for p in self.pos.iter()
        {
            paints.push(PaintPoint::new(p.clone(), Some('@')));
        }
        println!("{}", render(&paints));
    }
}

fn part_1(input: &str) -> usize
{
    let initial = State::initial(input);
    initial.display();

    if let Some((path, cost)) = pathfinding::directed::astar::astar(
        &initial,
        |s| s.next_taken_keys(),
        |_s| 0,
        |s| s.has_got_all_keys())
    {
        for p in path
        {
            p.display();
        }
        return cost;
    }
    assert!(false);
    unreachable!();
}

fn part_2(input: &str) -> usize
{
    let mut map = Map::new(input);

    let point = map.pos_of_key('@');

    map.set_char_at(&Point::new(point.x - 1, point.y - 1), '@');
    map.set_char_at(&Point::new(point.x + 0, point.y - 1), '#');
    map.set_char_at(&Point::new(point.x + 1, point.y - 1), '@');
    map.set_char_at(&Point::new(point.x - 1, point.y + 0), '#');
    map.set_char_at(&Point::new(point.x + 0, point.y + 0), '#');
    map.set_char_at(&Point::new(point.x + 1, point.y + 0), '#');
    map.set_char_at(&Point::new(point.x - 1, point.y + 1), '@');
    map.set_char_at(&Point::new(point.x + 0, point.y + 1), '#');
    map.set_char_at(&Point::new(point.x + 1, point.y + 1), '@');

    part_1(&map.chars.drain(..).collect::<String>())
}

fn main()
{
    const INPUT: &str = include_str!("input_18.txt");

    const EXAMPLE_1_1: &str = "#########\n#b.A.@.a#\n#########\n";
    const EXAMPLE_1_2: &str = "########################\n#f.D.E.e.C.b.A.@.a.B.c.#\n######################.#\n#d.....................#\n########################\n";
    const EXAMPLE_1_3: &str = "########################\n#...............b.C.D.f#\n#.######################\n#.....@.a.B.c.d.A.e.F.g#\n########################\n";
    const EXAMPLE_1_4: &str = "#################\n#i.G..c...e..H.p#\n########.########\n#j.A..b...f..D.o#\n########@########\n#k.E..a...g..B.n#\n########.########\n#l.F..d...h..C.m#\n#################\n";
    const EXAMPLE_1_5: &str = "########################\n#@..............ac.GI.b#\n###d#e#f################\n###A#B#C################\n###g#h#i################\n########################\n";

    println!("WARNING: This algorithm is too slow!");

    assert_eq!(part_1(EXAMPLE_1_1), 8);
    assert_eq!(part_1(EXAMPLE_1_2), 86);
    assert_eq!(part_1(EXAMPLE_1_3), 132);
    assert_eq!(part_1(EXAMPLE_1_4), 136);
    assert_eq!(part_1(EXAMPLE_1_5), 81);

    let answer_1 = part_1(INPUT);
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 4900);

    const EXAMPLE_2_1: &str = "#######\n#a.#Cd#\n##@#@##\n#######\n##@#@##\n#cB#.b#\n#######\n";
    const EXAMPLE_2_2: &str = "###############\n#d.ABC.#.....a#\n######@#@######\n###############\n######@#@######\n#b.....#.....c#\n###############\n";
    const EXAMPLE_2_3: &str = "#############\n#DcBa.#.GhKl#\n#.###@#@#I###\n#e#d#####j#k#\n###C#@#@###J#\n#fEbA.#.FgHi#\n#############\n";
    const EXAMPLE_2_4: &str = "#############\n#g#f.D#..h#l#\n#F###e#E###.#\n#dCba@#@BcIJ#\n#############\n#nK.L@#@G...#\n#M###N#H###.#\n#o#m..#i#jk.#\n#############\n";

    assert_eq!(part_1(EXAMPLE_2_1), 8);
    assert_eq!(part_1(EXAMPLE_2_2), 24);
    assert_eq!(part_1(EXAMPLE_2_3), 32);
    assert_eq!(part_1(EXAMPLE_2_4), 72);

    let answer_2 = part_2(INPUT);
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 2462);
}