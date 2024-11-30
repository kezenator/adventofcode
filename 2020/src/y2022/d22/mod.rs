use crate::support::*;
use itertools::*;
use pathfinding::utils::absdiff;

const EXAMPLE: &str = include_str!("example.txt");

#[derive(Copy, Clone, Debug)]
enum Turn
{
    Right,
    OneEighty,
    Left,
}

impl Turn
{
    fn num_right_turns(&self) -> i64
    {
        match self
        {
            Turn::Right => 1,
            Turn::OneEighty => 2,
            Turn::Left => 3,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Dir
{
    Right,
    Down,
    Left,
    Up,
}

impl Dir
{
    fn turn(&self, turn: Turn) -> Self
    {
        Self::from_int((self.to_int() + turn.num_right_turns()) % 4)
    }

    fn to_int(&self) -> i64
    {
        match self
        {
            Dir::Right => 0,
            Dir::Down => 1,
            Dir::Left => 2,
            Dir::Up => 3,
        }
    }

    fn from_int(val: i64) -> Self
    {
        match val
        {
            0 => Dir::Right,
            1 => Dir::Down,
            2 => Dir::Left,
            3 => Dir::Up,
            _ => unreachable!(),
        }
    }

    fn to_point_offset(&self) -> Point
    {
        match self
        {
            Dir::Right => Point::new(1, 0),
            Dir::Down => Point::new(0, 1),
            Dir::Left => Point::new(-1, 0),
            Dir::Up => Point::new(0, -1),
        }
    }
}

#[derive(Debug)]
struct Move
{
    dist: i64,
    turn: Option<Turn>,
}

#[derive(Copy, Clone, Debug)]
struct FacePoint
{
    face: i64,
    point: Point,
}

trait Layout
{
    fn face_size(&self) -> i64;
    fn fp_to_gp(&self, fp: &FacePoint) -> Point;
    fn fold_info(&self, face: i64, dir: Dir) -> (i64, Dir, bool);

    fn needs_fold(&self, pos: &FacePoint) -> bool
    {
        let size = self.face_size();

        pos.point.x < 0
        || pos.point.y < 0
        || pos.point.x >= size
        || pos.point.y >= size
    }

    fn fold(&self, pos: FacePoint, dir: Dir) -> (FacePoint, Dir)
    {
        let size = self.face_size();

        let (dest_face, dest_dir, reverse_coord) =
            self.fold_info(pos.face, dir);

        let mut coord = match dir
        {
            Dir::Right => pos.point.y,
            Dir::Down => pos.point.x,
            Dir::Left => pos.point.y,
            Dir::Up => pos.point.x,
        };

        if reverse_coord
        {
            coord = size - 1 - coord;
        }

        let dest_point = match dest_dir
        {
            Dir::Right => Point::new(0, coord),
            Dir::Down => Point::new(coord, 0),
            Dir::Left => Point::new(size - 1, coord),
            Dir::Up => Point::new(coord, size - 1),
        };

        let dest_fp = FacePoint
        {
            face: dest_face,
            point: dest_point,
        };

        (dest_fp, dest_dir)
    }
}

struct ExampleLayout
{
    size: i64,
    part2: bool
}

impl ExampleLayout
{
    fn new(size: i64, part2: bool) -> Self
    {
        ExampleLayout { size, part2 }
    }
}

impl Layout for ExampleLayout
{
    fn face_size(&self) -> i64
    {
        self.size
    }

    fn fp_to_gp(&self, fp: &FacePoint) -> Point
    {
        match fp.face
        {
            0 => fp.point + self.size * Point::new(2, 0),
            1 => fp.point + self.size * Point::new(0, 1),
            2 => fp.point + self.size * Point::new(1, 1),
            3 => fp.point + self.size * Point::new(2, 1),
            4 => fp.point + self.size * Point::new(2, 2),
            5 => fp.point + self.size * Point::new(3, 2),
            _ => unreachable!(),
        }
    }

    fn fold_info(&self, face: i64, dir: Dir) -> (i64, Dir, bool)
    {
        if !self.part2 // - wrap net
        {
            match (face, dir)
            {
                (0, Dir::Right) => (0, Dir::Right, false),
                (0, Dir::Down)  => (3, Dir::Down, false),
                (0, Dir::Left)  => (0, Dir::Left, false),
                (0, Dir::Up)    => (4, Dir::Up, false),
                (1, Dir::Right) => (2, Dir::Right, false),
                (1, Dir::Down)  => (1, Dir::Down, false),
                (1, Dir::Left)  => (3, Dir::Left, false),
                (1, Dir::Up)    => (1, Dir::Up, false),
                (2, Dir::Right) => (3, Dir::Right, false),
                (2, Dir::Down)  => (2, Dir::Down, false),
                (2, Dir::Left)  => (1, Dir::Left, false),
                (2, Dir::Up)    => (2, Dir::Up, false),
                (3, Dir::Right) => (1, Dir::Right, false),
                (3, Dir::Down)  => (4, Dir::Down, false),
                (3, Dir::Left)  => (2, Dir::Left, false),
                (3, Dir::Up)    => (0, Dir::Up, false),
                (4, Dir::Right) => (5, Dir::Right, false),
                (4, Dir::Down)  => (0, Dir::Down, false),
                (4, Dir::Left)  => (5, Dir::Left, false),
                (4, Dir::Up)    => (3, Dir::Up, false),
                (5, Dir::Right) => (4, Dir::Right, false),
                (5, Dir::Down)  => (5, Dir::Down, false),
                (5, Dir::Left)  => (4, Dir::Left, false),
                (5, Dir::Up)    => (5, Dir::Up, false),
                _ => unreachable!(),
            }
        }
        else // part2 - fold into a cube
        {
            match (face, dir)
            {
                (0, Dir::Right) => (5, Dir::Left, true),
                (0, Dir::Down)  => (3, Dir::Down, false),
                (0, Dir::Left)  => (2, Dir::Down, false),
                (0, Dir::Up)    => (1, Dir::Down, true),
                (1, Dir::Right) => (2, Dir::Right, false),
                (1, Dir::Down)  => (4, Dir::Up, true),
                (1, Dir::Left)  => (5, Dir::Up, true),
                (1, Dir::Up)    => (0, Dir::Down, true),
                (2, Dir::Right) => (3, Dir::Right, false),
                (2, Dir::Down)  => (4, Dir::Right, true),
                (2, Dir::Left)  => (1, Dir::Left, false),
                (2, Dir::Up)    => (0, Dir::Right, false),
                (3, Dir::Right) => (5, Dir::Down, true),
                (3, Dir::Down)  => (4, Dir::Down, false),
                (3, Dir::Left)  => (2, Dir::Left, false),
                (3, Dir::Up)    => (0, Dir::Up, false),
                (4, Dir::Right) => (5, Dir::Right, false),
                (4, Dir::Down)  => (1, Dir::Up, true),
                (4, Dir::Left)  => (2, Dir::Up, true),
                (4, Dir::Up)    => (3, Dir::Up, false),
                (5, Dir::Right) => (0, Dir::Left, true),
                (5, Dir::Down)  => (1, Dir::Right, true),
                (5, Dir::Left)  => (4, Dir::Left, false),
                (5, Dir::Up)    => (3, Dir::Left, true),
                _ => unreachable!(),
            }
        }
    }
}

struct MyInputLayout
{
    size: i64,
    part2: bool
}

impl MyInputLayout
{
    fn new(size: i64, part2: bool) -> Self
    {
        MyInputLayout { size, part2 }
    }
}

impl Layout for MyInputLayout
{
    fn face_size(&self) -> i64
    {
        self.size
    }

    fn fp_to_gp(&self, fp: &FacePoint) -> Point
    {
        match fp.face
        {
            0 => fp.point + self.size * Point::new(1, 0),
            1 => fp.point + self.size * Point::new(2, 0),
            2 => fp.point + self.size * Point::new(1, 1),
            3 => fp.point + self.size * Point::new(0, 2),
            4 => fp.point + self.size * Point::new(1, 2),
            5 => fp.point + self.size * Point::new(0, 3),
            _ => unreachable!(),
        }
    }

    fn fold_info(&self, face: i64, dir: Dir) -> (i64, Dir, bool)
    {
        if !self.part2 // - wrap net
        {
            match (face, dir)
            {
                (0, Dir::Right) => (1, Dir::Right, false),
                (0, Dir::Down)  => (2, Dir::Down, false),
                (0, Dir::Left)  => (1, Dir::Left, false),
                (0, Dir::Up)    => (4, Dir::Up, false),
                (1, Dir::Right) => (0, Dir::Right, false),
                (1, Dir::Down)  => (1, Dir::Down, false),
                (1, Dir::Left)  => (0, Dir::Left, false),
                (1, Dir::Up)    => (1, Dir::Up, false),
                (2, Dir::Right) => (2, Dir::Right, false),
                (2, Dir::Down)  => (4, Dir::Down, false),
                (2, Dir::Left)  => (2, Dir::Left, false),
                (2, Dir::Up)    => (0, Dir::Up, false),
                (3, Dir::Right) => (4, Dir::Right, false),
                (3, Dir::Down)  => (5, Dir::Down, false),
                (3, Dir::Left)  => (4, Dir::Left, false),
                (3, Dir::Up)    => (5, Dir::Up, false),
                (4, Dir::Right) => (3, Dir::Right, false),
                (4, Dir::Down)  => (0, Dir::Down, false),
                (4, Dir::Left)  => (3, Dir::Left, false),
                (4, Dir::Up)    => (2, Dir::Up, false),
                (5, Dir::Right) => (5, Dir::Right, false),
                (5, Dir::Down)  => (3, Dir::Down, false),
                (5, Dir::Left)  => (5, Dir::Left, false),
                (5, Dir::Up)    => (3, Dir::Up, false),
                _ => unreachable!(),
            }
        }
        else // part2 - fold into a cube
        {
            match (face, dir)
            {
                (0, Dir::Right) => (1, Dir::Right, false),
                (0, Dir::Down)  => (2, Dir::Down, false),
                (0, Dir::Left)  => (3, Dir::Right, true),
                (0, Dir::Up)    => (5, Dir::Right, false),
                (1, Dir::Right) => (4, Dir::Left, true),
                (1, Dir::Down)  => (2, Dir::Left, false),
                (1, Dir::Left)  => (0, Dir::Left, false),
                (1, Dir::Up)    => (5, Dir::Up, false),
                (2, Dir::Right) => (1, Dir::Up, false),
                (2, Dir::Down)  => (4, Dir::Down, false),
                (2, Dir::Left)  => (3, Dir::Down, false),
                (2, Dir::Up)    => (0, Dir::Up, false),
                (3, Dir::Right) => (4, Dir::Right, false),
                (3, Dir::Down)  => (5, Dir::Down, false),
                (3, Dir::Left)  => (0, Dir::Right, true),
                (3, Dir::Up)    => (2, Dir::Right, false),
                (4, Dir::Right) => (1, Dir::Left, true),
                (4, Dir::Down)  => (5, Dir::Left, false),
                (4, Dir::Left)  => (3, Dir::Left, false),
                (4, Dir::Up)    => (2, Dir::Up, false),
                (5, Dir::Right) => (4, Dir::Up, false),
                (5, Dir::Down)  => (1, Dir::Down, false),
                (5, Dir::Left)  => (0, Dir::Down, false),
                (5, Dir::Up)    => (3, Dir::Up, false),
                _ => unreachable!(),
            }
        }
    }
}

fn parse(input: &str) -> (CharGrid, Vec<Move>)
{
    let mut groups = input_to_groups(input);

    let max_line_length = groups[0].iter()
        .map(|l| l.len())
        .max()
        .unwrap();

    for l in groups[0].iter_mut()
    {
        if l.len() < max_line_length
        {
            l.extend(vec![' ';max_line_length - l.len()]);
        }
    }

    let grid = CharGrid::new_from_input(&groups[0].join("\n"), ' ');

    let mut moves = Vec::new();
    let chars = groups[1][0].chars().collect_vec();
    let mut index = 0;
    while index < chars.len()
    {
        let start = index;
        while (index < chars.len()) && chars[index].is_digit(10) { index += 1; }

        let dist = chars[start..index].iter().collect::<String>().parse::<i64>().unwrap();
        let mut turn = None;
        if index < chars.len()
        {
            turn = match chars[index]
            {
                'R' => Some(Turn::Right),
                'L' => Some(Turn::Left),
                _ => unreachable!(),
            };
            index += 1;
        }
        moves.push(Move{ dist, turn });
    }

    (grid, moves)
}

fn check_cube_folding(layout: &dyn Layout)
{
    // First - check that matching
    // fold into edges line up

    for face in 0..6
    {
        for dir in [Dir::Right, Dir::Down, Dir::Left, Dir::Up]
        {
            let (dest_face, dest_dir, reverse_coords)
                = layout.fold_info(face, dir);

            let (src_face, src_dir, src_reverse)
                = layout.fold_info(dest_face, dest_dir.turn(Turn::OneEighty));

            //println!("Check: {}/{:?} => {}/{:?}/{} => {}/{:?}/{}",
            //    face, dir,
            //    dest_face, dest_dir, reverse_coords,
            //    src_face, src_dir, src_reverse);

            assert!(src_face == face);
            assert!(src_dir == dir.turn(Turn::OneEighty));
            assert!(reverse_coords == src_reverse);
        }
    }

    // Now - check that 4*size steps in each direction gets
    // us back to the origin

    let size = layout.face_size();

    for dir in [Dir::Right, Dir::Down, Dir::Left, Dir::Up]
    {
        let mut cur_pos = FacePoint{ face: 0, point: Point::new(0, 0), };
        let mut cur_dir = dir;

        //println!("Check folding from {:?}/{:?}", cur_pos, cur_dir);

        for _ in 0..(4*size)
        {
            let mut next_pos = FacePoint
            {
                face: cur_pos.face,
                point: cur_pos.point + cur_dir.to_point_offset()
            };
            let mut next_dir = cur_dir;

            if layout.needs_fold(&next_pos)
            {
                (next_pos, next_dir) = layout.fold(next_pos, next_dir);
                
                //println!("    Folded: {:?}/{:?} => {:?}/{:?}",
                //    cur_pos, cur_dir, next_pos, next_dir);
            }

            cur_pos = next_pos;
            cur_dir = next_dir;
        }
        //println!("    Final: {:?}/{:?}", cur_pos, cur_dir);

        assert!(cur_pos.face == 0);
        assert!(cur_pos.point.x == 0);
        assert!(cur_pos.point.y == 0);
        assert!(cur_dir == dir);
    }
}

fn get_password(input: &str, part2: bool) -> i64
{
    let (grid, moves) = parse(input);

    // Work out which layout to use

    let layout: Box<dyn Layout>;
    if grid.get_width() == 16
    {
        layout = Box::new(ExampleLayout::new(4, part2));
    }
    else
    {
        layout = Box::new(MyInputLayout::new(50, part2));
    }

    // Check the cube folding is correct

    if part2
    {
        check_cube_folding(layout.as_ref());
    }

    // Work out the starting pos

    let mut cur_pos = FacePoint { face: 0, point: Point::new(0, 0), };

    while grid.get_char(&layout.fp_to_gp(&cur_pos)) == '#'
    {
        cur_pos.point.x += 1;
    }

    // Perform the moves

    let mut cur_dir = Dir::Right;

    for mv in moves
    {
        //println!("Move: {:?}", mv);
        //println!("    At: {:?}/{:?}", cur_pos, cur_dir);

        for _ in 0..mv.dist
        {
            let mut next_pos = FacePoint
            {
                face: cur_pos.face,
                point: cur_pos.point + cur_dir.to_point_offset(),
            };
            let mut next_dir = cur_dir;

            //println!("   Basic Move: {:?}", next_pos);

            if layout.needs_fold(&next_pos)
            {
                (next_pos, next_dir) = layout.fold(next_pos, cur_dir);
            }

            //println!("    Folded: {:?} / {:?}", next_pos, next_dir);

            let next_gp = layout.fp_to_gp(&next_pos);
            let next_char = grid.get_char(&next_gp);

            //println!("    => {:?} => '{}'", next_gp, next_char);

            match next_char
            {
                '.' => (cur_pos, cur_dir) = (next_pos, next_dir),
                '#' => break,
                _ => unreachable!(),
            }
        }

        if let Some(turn) = mv.turn
        {
            cur_dir = cur_dir.turn(turn);

            //println!("    Turning to {:?}", cur_dir);
        }

        //println!("    Now: {:?}/{:?}", cur_pos, cur_dir);
    }

    let final_gp = layout.fp_to_gp(&cur_pos);

    //println!("Final: {:?} / {:?} => Grid {:?}", cur_pos, cur_dir, final_gp);

    (1000 * (final_gp.y + 1)) + (4 * (final_gp.x + 1)) + cur_dir.to_int()
}

fn part_1(input: &str) -> i64
{
    get_password(input, false)
}

fn part_2(input: &str) -> i64
{
    get_password(input, true)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(22)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 6032,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 162186,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 5031,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 55267,
        })
}
