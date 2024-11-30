use std::collections::{ HashMap, HashSet };
use crate::support::*;

struct Map
{
    doors: HashMap<Point, HashSet<Point>>,
}

impl Map
{
    pub fn new(input: &str) -> Self
    {
        let mut result = Map { doors: HashMap::new() };

        let line = input_to_lines(input)[0].clone();
        let mut chars = line.chars();

        // Take out the initial '^' - it's handled by us
        // calling parse below. This function will then return
        // on the '$' character.

        assert_eq!(chars.next(), Some('^'));

        // Parse the string, starting at (0, 0)
        
        let _ = result.parse(
            &mut chars,
            vec![ Point::new(0, 0) ]);

        // Check we've got to the end of the string

        assert_eq!(chars.next(), None);

        result
    }

    fn parse<I>(&mut self, chars: &mut I, starting_points: Vec<Point>) -> Vec<Point>
        where I: Iterator<Item=char>
    {
        // Parses between a set of parenthesis "()" or "^$"
        // Takes a set of points where we currently are.
        // Returns a set of points where we stop.

        // Our current location is the starting locations

        let mut cur_points = starting_points.clone();

        // This is to build up the set of unique points we end up with.
        // It's a set so that paths like (NS|EW) will cancel out and still only
        // return one end position.

        let mut or_end_points = HashSet::new();

        // Helper to perform a N/S/E/W move

        let move_step = |map: &mut Map, cur_points: &mut Vec<Point>, dir: Point|
        {
            // For each point in the set of current points

            for p in cur_points.iter_mut()
            {
                let dest = *p + dir;

                // Insert a door in each direction

                map.doors.entry(p.clone()).or_default().insert(dest.clone());
                map.doors.entry(dest.clone()).or_default().insert(p.clone());

                // Update the cur_points array

                *p = dest;
            }
        };

        loop
        {
            match chars.next().unwrap()
            {
                '^' | '(' =>
                {
                    // Parse a whole set of parenthesis.
                    // We are now at their ending position

                    cur_points = self.parse(chars, cur_points.clone());
                },
                '$' | ')' =>
                {
                    // We've finished this set of parenthesis
                    // 1) Add each currently location to the set of end locations.
                    // 2) Return the complete set of end locations - with any
                    //    duplicates removed (e.g. (NS|EW) will return one final location).

                    for p in cur_points
                    {
                        or_end_points.insert(p);
                    }

                    return or_end_points.drain().collect::<Vec<_>>();
                },
                '|' =>
                {
                    // We've completed one branch of an or operation.
                    // 1) Add each current location to the set of end locations
                    //    we will eventually return at the ")"
                    // 2) Reset our current location back to the starting location
                    //    from the "("

                    for p in cur_points
                    {
                        or_end_points.insert(p);
                    }

                    cur_points = starting_points.clone();
                },

                // Simple movements

                'N' => move_step(self, &mut cur_points, Point::new(0, 1)),
                'S' => move_step(self, &mut cur_points, Point::new(0, -1)),
                'E' => move_step(self, &mut cur_points, Point::new(1, 0)),
                'W' => move_step(self, &mut cur_points, Point::new(-1, 0)),
                _ => unreachable!(),
            }

            //println!("{} => {:?}", ch, cur_points);
        }
    }

    pub fn distance_to_each_room(&self) -> HashMap<Point, i64>
    {
        let mut distances = HashMap::new();
        let mut remaining_rooms = self.doors.keys().cloned().collect::<HashSet<_>>();
        let mut edge = HashSet::new();
        let mut cur_distance = 0;

        // Insert that (0, 0) is distance 0,
        // it's not remaining, and it's the current edge
        // that we're advancing out from

        distances.insert(Point::new(0, 0), cur_distance);
        remaining_rooms.remove(&Point::new(0, 0));
        edge.insert(Point::new(0, 0));

        // While we can advance out further

        while !edge.is_empty()
        {
            cur_distance += 1;

            let mut new_edge = HashSet::new();

            for p in edge
            {
                // For each room we can move to from here

                for np in self.doors.get(&p).unwrap().iter()
                {
                    // If we've not moved to this room before,
                    // then record it's shortest distance,
                    // say it's not remaining, and insert it into
                    // the new edge of advancement.

                    if remaining_rooms.contains(np)
                    {
                        distances.insert(np.clone(), cur_distance);
                        remaining_rooms.remove(np);
                        new_edge.insert(np.clone());
                    }
                }
            }

            edge = new_edge;
        }

        assert!(remaining_rooms.is_empty());
        distances
    }
}

pub fn part_1(input: &str) -> i64
{
    Map::new(input)
        .distance_to_each_room()
        .values()
        .cloned()
        .max()
        .unwrap()
        .clone()
}

fn part_2(input: &str) -> usize
{
    Map::new(input)
        .distance_to_each_room()
        .iter()
        .filter(|(_p, &d)| d >= 1000)
        .count()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(20)
        .example(|| Answer { calculated: part_1("^WNE$"), expected: 3, })
        .example(|| Answer { calculated: part_1("^ENWWW(NEEE|SSE(EE|N))$"), expected: 10, })
        .example(|| Answer { calculated: part_1("^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$"), expected: 18, })
        .example(|| Answer { calculated: part_1("^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$"), expected: 23, })
        .example(|| Answer { calculated: part_1("^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$"), expected: 31, })
        .part_1(|input| Answer { calculated: part_1(input), expected: 3930, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 8240, })
}
