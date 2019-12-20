use aoc2019::*;
use std::collections::HashMap;

#[derive(Debug)]
enum PathType
{
    Steps,
    PortalInner,
    PortalOuter,
}

#[derive(Debug)]
struct Map
{
    chars: Vec<char>,
    width: i64,
    height: i64,
    portals: HashMap<Point, String>,
    initial_pos: Point,
    goal: Point,
    recursive: bool,
    moves: HashMap<Point, Vec<(Point, PathType, usize)>>,
}

impl Map
{
    fn new(input: &str, recursive: bool) -> Self
    {
        let chars = input.chars().collect::<Vec<char>>();
        let width = input.split("\n").nth(0).unwrap().chars().count() as i64;
        let height = (input.chars().count() as i64) / (width + 1);
        let portals = HashMap::new();
        let initial_pos = Point::new(0, 0);
        let goal = Point::new(0, 0);
        let moves = HashMap::new();

        let mut result = Map { chars, width, height, portals, initial_pos, goal, recursive, moves };

        result.find_portals();

        result.initial_pos = result.portals.iter()
            .filter(|(_, s)| **s == "AA".to_string())
            .map(|(p, _)| p.clone())
            .nth(0)
            .unwrap();
        result.goal = result.portals.iter()
            .filter(|(_, s)| **s == "ZZ".to_string())
            .map(|(p, _)| p.clone())
            .nth(0)
            .unwrap();

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

    fn is_outer(&self, point: &Point) -> bool
    {
        point.x < 3 || point.y < 3 || point.x >= (self.width - 3) || point.y >= (self.height - 3)
    }

    fn neighbours(&self, point: &Point) -> Vec<Point>
    {
        point.cardinal_neighbours()
            .drain(..)
            .filter(|p| self.char_at(p) == '.')
            .collect::<Vec<_>>()
    }

    fn find_portals(&mut self)
    {
        for y in 0..self.height
        {
            for x in 0..self.width
            {
                let point = Point::new(x, y);
                if self.char_at(&point) == '.'
                {
                    for n in point.cardinal_neighbours()
                    {
                        let ochar = self.char_at(&n);
                        if ochar >= 'A' && ochar <= 'Z'
                        {
                            let spoint = n.add(&n.subtract(&point));
                            let schar = self.char_at(&spoint);

                            let name = if spoint.x > n.x || spoint.y > n.y
                            {
                                format!("{}{}", ochar, schar)
                            }
                            else
                            {
                                format!("{}{}", schar, ochar)
                            };

                            //println!("{:?} => {}", point, name);

                            self.portals.insert(point, name);
                        }
                    }
                }
            }
        }
    }

    fn generate_moves(&mut self)
    {
        let mut _total_moves = 0;

        for (&from, from_str) in self.portals.iter()
        {
            let mut to = Vec::new();

            for (&trial, trial_str) in self.portals.iter()
            {
                if trial != from
                {
                    // Zoom between matching portals with one step,
                    // or find the shortest path between them

                    if *from_str == *trial_str
                    {
                        _total_moves += 1;

                        if self.is_outer(&from)
                        {
                            to.push((trial, PathType::PortalOuter, 1));
                        }
                        else
                        {
                            to.push((trial, PathType::PortalInner, 1));
                        }
                    }
                    else if let Some((_path, cost)) = pathfinding::directed::astar::astar(
                        &from,
                        |p| self.neighbours(p).iter()
                                .map(|np| (np.clone(), 1 as usize))
                                .collect::<Vec<_>>(),
                        |p| p.manhatten_dist_to(&trial) as usize,
                        |p| *p == trial)
                    {
                        _total_moves += 1;
                        to.push((trial, PathType::Steps, cost));
                    }
                }
            }

            //println!("{:?} => {:?}", from, to);

            self.moves.insert(from, to);
        }

        //println!("Collected {} moves", _total_moves);
    }

    fn shortest_path(&self) -> usize
    {
        let initial = (self.initial_pos.clone(), 0);
        let goal = (self.goal.clone(), 0);

        let get_moves = |from: Point, level: usize| -> Vec<((Point, usize), usize)>
        {
            let mut result = Vec::new();

            for (dest, path_type, cost) in self.moves.get(&from).unwrap().iter()
            {
                if self.recursive
                {
                    match *path_type
                    {
                        PathType::Steps =>
                        {
                            // We can always walk along paths
                            result.push(((dest.clone(), level), *cost));
                        },
                        PathType::PortalInner =>
                        {
                            // We can always decend into a lower layer
                            result.push(((dest.clone(), level + 1), *cost));
                        },
                        PathType::PortalOuter =>
                        {
                            // At the outer layer, we can only
                            // exit at ZZ. In inner layers, we can only
                            // use other portals to exit to upper layers.

                            let is_zz = *self.portals.get(dest).unwrap() == "ZZ".to_string();

                            if level == 0
                            {
                                if is_zz
                                {
                                    result.push(((dest.clone(), 0), *cost));
                                }
                            }
                            else
                            {
                                if !is_zz
                                {
                                    result.push(((dest.clone(), level - 1), *cost));
                                }
                            }
                        },
                    }
                }
                else // not recursive
                {
                    // Just take every move, staying at the top level
                    result.push(((dest.clone(), 0), *cost));
                }
            }

            result
        };

        if let Some((_path, cost)) = pathfinding::directed::astar::astar(
            &initial,
            |(p, l)| get_moves(p.clone(), *l),
            |_| 0,
            |pl| *pl == goal)
        {
            //println!("{:?}", _path);
            return cost;
        }
        assert!(false);
        unreachable!();
    }
}

fn part_1(input: &str) -> usize
{
    let map = Map::new(input, false);
    map.shortest_path()
}

fn part_2(input: &str) -> usize
{
    let map = Map::new(input, true);
    map.shortest_path()
}

fn main()
{
    const INPUT: &str = include_str!("input_20.txt");

    const EXAMPLE_1_1: &str = "         A           \n         A           \n  #######.#########  \n  #######.........#  \n  #######.#######.#  \n  #######.#######.#  \n  #######.#######.#  \n  #####  B    ###.#  \nBC...##  C    ###.#  \n  ##.##       ###.#  \n  ##...DE  F  ###.#  \n  #####    G  ###.#  \n  #########.#####.#  \nDE..#######...###.#  \n  #.#########.###.#  \nFG..#########.....#  \n  ###########.#####  \n             Z       \n             Z       \n";
    const EXAMPLE_1_2: &str = "                   A               \n                   A               \n  #################.#############  \n  #.#...#...................#.#.#  \n  #.#.#.###.###.###.#########.#.#  \n  #.#.#.......#...#.....#.#.#...#  \n  #.#########.###.#####.#.#.###.#  \n  #.............#.#.....#.......#  \n  ###.###########.###.#####.#.#.#  \n  #.....#        A   C    #.#.#.#  \n  #######        S   P    #####.#  \n  #.#...#                 #......VT\n  #.#.#.#                 #.#####  \n  #...#.#               YN....#.#  \n  #.###.#                 #####.#  \nDI....#.#                 #.....#  \n  #####.#                 #.###.#  \nZZ......#               QG....#..AS\n  ###.###                 #######  \nJO..#.#.#                 #.....#  \n  #.#.#.#                 ###.#.#  \n  #...#..DI             BU....#..LF\n  #####.#                 #.#####  \nYN......#               VT..#....QG\n  #.###.#                 #.###.#  \n  #.#...#                 #.....#  \n  ###.###    J L     J    #.#.###  \n  #.....#    O F     P    #.#...#  \n  #.###.#####.#.#####.#####.###.#  \n  #...#.#.#...#.....#.....#.#...#  \n  #.#####.###.###.#.#.#########.#  \n  #...#.#.....#...#.#.#.#.....#.#  \n  #.###.#####.###.###.#.#.#######  \n  #.#.........#...#.............#  \n  #########.###.###.#############  \n           B   J   C               \n           U   P   P               \n";

    assert_eq!(part_1(EXAMPLE_1_1), 23);
    assert_eq!(part_1(EXAMPLE_1_2), 58);

    let answer_1 = part_1(INPUT);
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 462);

    const EXAMPLE_2_1: &str = "             Z L X W       C                 \n             Z P Q B       K                 \n  ###########.#.#.#.#######.###############  \n  #...#.......#.#.......#.#.......#.#.#...#  \n  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  \n  #.#...#.#.#...#.#.#...#...#...#.#.......#  \n  #.###.#######.###.###.#.###.###.#.#######  \n  #...#.......#.#...#...#.............#...#  \n  #.#########.#######.#.#######.#######.###  \n  #...#.#    F       R I       Z    #.#.#.#  \n  #.###.#    D       E C       H    #.#.#.#  \n  #.#...#                           #...#.#  \n  #.###.#                           #.###.#  \n  #.#....OA                       WB..#.#..ZH\n  #.###.#                           #.#.#.#  \nCJ......#                           #.....#  \n  #######                           #######  \n  #.#....CK                         #......IC\n  #.###.#                           #.###.#  \n  #.....#                           #...#.#  \n  ###.###                           #.#.#.#  \nXF....#.#                         RF..#.#.#  \n  #####.#                           #######  \n  #......CJ                       NM..#...#  \n  ###.#.#                           #.###.#  \nRE....#.#                           #......RF\n  ###.###        X   X       L      #.#.#.#  \n  #.....#        F   Q       P      #.#.#.#  \n  ###.###########.###.#######.#########.###  \n  #.....#...#.....#.......#...#.....#.#...#  \n  #####.#.###.#######.#######.###.###.#.#.#  \n  #.......#.......#.#.#.#.#...#...#...#.#.#  \n  #####.###.#####.#.#.#.#.###.###.#.###.###  \n  #.......#.....#.#...#...............#...#  \n  #############.#.#.###.###################  \n               A O F   N                     \n               A A D   M                     \n";

    assert_eq!(part_2(EXAMPLE_2_1), 396);

    let answer_2 = part_2(INPUT);
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 5288);
}