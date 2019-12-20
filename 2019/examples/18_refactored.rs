use aoc2019::*;
use std::rc::Rc;
use std::collections::{BTreeSet, HashMap};

#[derive(Debug)]
struct Map
{
    chars: Vec<char>,
    width: i64,
    height: i64,
    all_keys: BTreeSet<char>,
    initial_pos: Vec<Point>,
    moves: HashMap<char, Vec<(char, usize)>>,
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
        let moves = HashMap::new();

        let mut result = Map { chars, width, height, all_keys, initial_pos, moves };
        result.generate_moves();
        result
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

    fn initial_pos_chars(&self) -> Vec<char>
    {
        let mut result = Vec::new();
        for i in 0..self.initial_pos.len()
        {
            result.push((('0' as u8) + (i as u8)) as char);
        }
        result
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
        if key >= '0' && key <= '9'
        {
            let index = format!("{}", key).parse::<usize>().unwrap();
            return *self.initial_pos.iter().nth(index).unwrap();
        }

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
        println!("Can't find pos of key '{}'", key);
        assert!(false);
        unreachable!();
    }

    fn generate_moves(&mut self)
    {
        let mut all_from = self.initial_pos_chars();
        for &key in self.all_keys.iter()
        {
            all_from.push(key);

            let door = key.to_ascii_uppercase();
            if self.chars.iter().filter(|&ch| *ch == door).count() > 0
            {
                all_from.push(door);
            }
        }

        let mut total_moves = 0;

        for &from in all_from.iter()
        {
            let from_pos = self.pos_of_key(from);
            let mut to = Vec::new();

            for &trial in all_from.iter()
            {
                if trial != from
                {
                    let trial_pos = self.pos_of_key(trial);

                    let mut trial_test = trial;
                    if trial >= '0' && trial <= '9'
                    {
                        trial_test = '@';
                    }

                    if let Some((_path, cost)) = pathfinding::directed::astar::astar(
                        &from_pos,
                        |p| self.neighbours(p).iter()
                                .filter(|(_np, nch)| *nch == trial_test || *nch == '.')
                                .map(|(np, _nch)| (np.clone(), 1 as usize))
                                .collect::<Vec<_>>(),
                        |p| p.manhatten_dist_to(&trial_pos) as usize,
                        |p| *p == trial_pos)
                    {
                        total_moves += 1;
                        to.push((trial, cost));
                    }
                }
            }

            println!("{} => {:?}", from, to);

            self.moves.insert(from, to);
        }

        println!("Collected {} moves", total_moves);
    }
}

#[derive(Debug, Clone)]
struct State
{
    map: Rc<Map>,
    pos: Vec<char>,
    keys: BTreeSet<char>,
    opened_doors: BTreeSet<char>,
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
        let pos = map.initial_pos_chars();
        let keys = BTreeSet::new();
        let opened_doors = BTreeSet::new();

        State { map, pos, keys, opened_doors }
    }

    fn neighbours(&self) -> Vec<(State, usize)>
    {
        let mut result = Vec::new();

        for i in 0..self.pos.len()
        {
            for (dest_ch, dest_cost) in self.map.moves.get(&self.pos[i]).unwrap().iter()
            {
                if *dest_ch >= 'A' && *dest_ch <= 'Z'
                {
                    // Door: We can move here if we have the key.
                    if self.keys.contains(&dest_ch.to_lowercase().nth(0).unwrap())
                    {
                        let mut new_pos = self.pos.clone();
                        new_pos[i] = *dest_ch;
                        let mut new_doors = self.opened_doors.clone();
                        new_doors.insert(*dest_ch);
                        let new_state = State
                        {
                            map: self.map.clone(),
                            pos: new_pos,
                            keys: self.keys.clone(),
                            opened_doors: new_doors,
                        };
                        result.push((new_state, *dest_cost));
                    }
                }
                else if *dest_ch >= 'a' && *dest_ch <= 'z'
                {
                    // Key: collect it
                    let mut new_pos = self.pos.clone();
                    new_pos[i] = *dest_ch;
                    let mut new_keys = self.keys.clone();
                    new_keys.insert(*dest_ch);
                    let new_state = State
                    {
                        map: self.map.clone(),
                        pos: new_pos,
                        keys: new_keys,
                        opened_doors: self.opened_doors.clone(),
                    };
                    result.push((new_state, *dest_cost));
                }
                else
                {
                    // Jus move here with no change
                    let mut new_pos = self.pos.clone();
                    new_pos[i] = *dest_ch;
                    let new_state = State
                    {
                        map: self.map.clone(),
                        pos: new_pos,
                        keys: self.keys.clone(),
                        opened_doors: self.opened_doors.clone(),
                    };
                    result.push((new_state, *dest_cost));
                }
            }
        }

        result
    }

    fn has_got_all_keys(&self) -> bool
    {
        self.keys.len() == self.map.all_keys.len()
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
            paints.push(PaintPoint::new(self.map.pos_of_key(*p), Some('@')));
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
        |s| s.neighbours(),
        |s| s.map.all_keys.len() - s.keys.len(),
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