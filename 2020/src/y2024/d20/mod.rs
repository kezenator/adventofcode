
use rayon::prelude::*;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

struct Map
{
    grid: CharGrid,
    start: Point,
    end: Point,
}

impl Map
{
    fn new(input: &str) -> Self
    {
        let grid = CharGrid::new_from_input(input, '#');
        let start = grid.all_points().into_iter()
            .filter(|p| grid.get_char(p) == 'S')
            .next().unwrap();
        let end = grid.all_points().into_iter()
            .filter(|p| grid.get_char(p) == 'E')
            .next().unwrap();

        Map { grid, start, end }
    }

    fn shortest_path(&self, cheat: Option<Point>) -> Vec<Point>
    {
        let astar_result = pathfinding::directed::astar::astar(
            &self.start,
            |cur|
            {
                cur.neighbours_4()
                    .filter(|next|
                        (self.grid.get_char(next) != '#')
                        || (cheat.is_some() && cheat.unwrap() == *next))
                    .map(|next| (next, 1))
            },
            |cur|
            {
                (*cur - self.end).manhatten_size()
            },
            |cur| *cur == self.end);
        
        astar_result.unwrap().0
    }

    fn count_shortcuts(&self, min_saving: usize, max_duration: i64) -> usize
    {
        assert!(max_duration > 0);
        assert!(min_saving >= (max_duration as usize));
        assert!(min_saving >= 2);

        // To a cheat:
        // 1) Cut's down the shortest path length (i.e. the cheat start and end
        //    are separated in the "shortest_path" vector by at least min_saving
        // 2) Is within the max duration (i.e. the cheat start and end are
        //    separated by at most max_duration in grid-space)

        let shortest_path = self.shortest_path(None);

        (0..shortest_path.len()).into_par_iter()
            .map(|cheat_si|
            {
                let mut num_shortcuts = 0;
                let cheat_s = shortest_path[cheat_si];

                // The minimum cheat is 2 long - so take 2
                for cheat_ei in (cheat_si + min_saving - 2)..shortest_path.len()
                {
                    let cheat_e = shortest_path[cheat_ei];
                    let dist = (cheat_s - cheat_e).manhatten_size();
                    if dist <= max_duration
                    {
                        // OK - we've got a potental point that is a viable short-cut both:
                        // 1: In terms of cutting down the path length
                        // 2: Achievable in the maximum duration
                        //
                        // Now - see if it saves anything...
    
                        let duration_to_start = cheat_si;
                        let duration_of_cheat = dist as usize;
                        let duration_to_end = shortest_path.len() - 1 - cheat_ei;
                        let total_duration = duration_to_start + duration_of_cheat + duration_to_end;
                        
                        if total_duration < shortest_path.len()
                        {
                            let saving = shortest_path.len() - total_duration;
    
                            if saving >= min_saving
                            {
                                num_shortcuts += 1;
                            }
                        }
                    }
                }
                num_shortcuts
            })
            .sum()
    }
}

fn part_1(input: &str, min_saving: usize) -> usize
{
    let map = Map::new(input);
    map.count_shortcuts(min_saving, 2)
}

fn part_2(input: &str, min_saving: usize) -> usize
{
    let map = Map::new(input);
    map.count_shortcuts(min_saving, 20)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(20)
        .example(|| Answer {
            calculated: part_1(EXAMPLE, 2),
            expected: 44,
        })
        .part_1(|input| Answer {
            calculated: part_1(input, 100),
            expected: 1381,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE, 50),
            expected: 285,
        })
        .part_2(|input| Answer {
            calculated: part_2(input, 100),
            expected: 982124,
        })
}
