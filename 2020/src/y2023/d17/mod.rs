use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");
const EXAMPLE2: &str = include_str!("example2.txt");

#[derive(Clone, Copy)]
enum CruicibleSize
{
    Normal,
    Ultra,
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct State
{
    pos: Point,
    dir: Point,
    steps_in_dir: usize,
}

impl State
{
    fn valid_steps_to_complete(&self, size: CruicibleSize) -> bool
    {
        match size
        {
            CruicibleSize::Normal => true,
            CruicibleSize::Ultra => self.steps_in_dir >= 4,
        }
    }

    fn neighbours(&self, grid: &CharGrid, size: CruicibleSize) -> Vec<(State, usize)>
    {
        let mut result = vec![];

        let can_go_straight;
        let can_turn;

        match size
        {
            CruicibleSize::Normal =>
            {
                // If not moved three steps, then try
                // continuing in the current direction

                can_go_straight = self.steps_in_dir < 3;
                can_turn = true;
            },
            CruicibleSize::Ultra =>
            {
                can_go_straight = self.steps_in_dir < 10;
                can_turn = self.steps_in_dir >= 4;
            },
        }

        if can_go_straight
        {
            self.try_add_dir(&mut result, grid, self.dir, self.steps_in_dir + 1);
        }

        if can_turn
        {
            self.try_add_dir(&mut result, grid, self.dir.rotate_90_left(), 1);
            self.try_add_dir(&mut result, grid, self.dir.rotate_90_right(), 1);
        }

        result
    }

    fn try_add_dir(&self, result: &mut Vec<(State, usize)>, grid: &CharGrid, new_dir: Point, steps: usize)
    {
        let new_pos = self.pos + new_dir;
        if grid.is_point_in_bounds(&new_pos)
        {
            let entry_heat_loss = (grid.get_char(&new_pos) as usize) - ('0' as usize);

            result.push((State{
                pos: new_pos,
                dir: new_dir,
                steps_in_dir: steps,
            }, entry_heat_loss));
        }
    }
}

fn find_best_path(input: &str, size: CruicibleSize) -> usize
{
    let grid = CharGrid::new_from_input(input, '1');
    let end_point = Point::new(grid.get_width() - 1, grid.get_height() - 1);
    let initial = State
    {
        pos: Point::new(0, 0),
        dir: Point::new(1, 0),
        steps_in_dir: 0,
    };

    let result = pathfinding::directed::astar::astar(
        &initial,
        |s| s.neighbours(&grid, size),
        |s| (s.pos - end_point).manhatten_size() as usize,
        |s| (s.pos == end_point) && s.valid_steps_to_complete(size));

    result.unwrap().1
}

fn part_1(input: &str) -> usize
{
    find_best_path(input, CruicibleSize::Normal)
}

fn part_2(input: &str) -> usize
{
    find_best_path(input, CruicibleSize::Ultra)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(17)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 102,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 1099,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 94,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE2),
            expected: 71,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 1266,
        })
}
