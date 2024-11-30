use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum MovesTaken
{
    NotMoved,
    InCoridor,
    Organized,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Kind
{
    Amber,
    Bronze,
    Copper,
    Desert,
}

impl Kind
{
    fn cost_per_step(&self) -> usize
    {
        match self
        {
            Kind::Amber => 1,
            Kind::Bronze => 10,
            Kind::Copper => 100,
            Kind::Desert => 1000,
        }
    }

    fn target_room_x(&self) -> i64
    {
        match self
        {
            Kind::Amber => 3,
            Kind::Bronze => 5,
            Kind::Copper => 7,
            Kind::Desert => 9,
        }
    }

    fn all_kinds() -> impl Iterator<Item = Kind>
    {
        [Kind::Amber, Kind::Bronze, Kind::Copper, Kind::Desert].iter().copied()
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Amphipod
{
    kind: Kind,
    moves_taken: MovesTaken,
    pos: Point,
}

impl Amphipod
{
    fn new(kind: Kind, pos: Point) -> Self
    {
        let moves_taken = MovesTaken::NotMoved;

        Amphipod { kind, moves_taken, pos }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Burrow
{
    max_y: i64,
    pods: Vec<Amphipod>,
}

impl Burrow
{
    fn new(input: &str, part_2: bool) -> Self
    {
        let grid = CharGrid::new_from_input(input, ' ');

        let mut pods = Vec::new();
        let mut max_y = 3;

        for pos in grid.all_points()
        {
            let ch = grid.get_char(&pos);

            match ch
            {
                'A' => pods.push(Amphipod::new(Kind::Amber, pos)),
                'B' => pods.push(Amphipod::new(Kind::Bronze, pos)),
                'C' => pods.push(Amphipod::new(Kind::Copper, pos)),
                'D' => pods.push(Amphipod::new(Kind::Desert, pos)),
                _ => (),
            };
        }

        if part_2
        {
            max_y = 5;

            // Move all items in row y=3 to y=5

            for pod in pods.iter_mut()
            {
                if pod.pos.y == 3
                {
                    pod.pos.y = 5;
                }
            }

            // Add in the revealed new 'pods

            pods.push(Amphipod::new(Kind::Desert, Point::new(3, 3)));
            pods.push(Amphipod::new(Kind::Copper, Point::new(5, 3)));
            pods.push(Amphipod::new(Kind::Bronze, Point::new(7, 3)));
            pods.push(Amphipod::new(Kind::Amber,  Point::new(9, 3)));

            pods.push(Amphipod::new(Kind::Desert, Point::new(3, 4)));
            pods.push(Amphipod::new(Kind::Bronze, Point::new(5, 4)));
            pods.push(Amphipod::new(Kind::Amber,  Point::new(7, 4)));
            pods.push(Amphipod::new(Kind::Copper, Point::new(9, 4)));
        }

        // Search up each room from the bottom and
        // mark already-organised 'pods

        for kind in Kind::all_kinds()
        {
            let x = kind.target_room_x();
            let mut y = max_y;

            while y >= 2
            {
                let pos = Point::new(x, y);

                if let Some(pod) = pods.iter_mut().filter(|pod| (pod.pos == pos) && (pod.kind == kind)).next()
                {
                    pod.moves_taken = MovesTaken::Organized;
                    y -= 1;
                    continue;
                }
                else
                {
                    // No more correct pods at the bottom of this row
                    break;
                }
            }
        }

        Burrow { max_y, pods }
    }

    fn is_fully_organised(&self) -> bool
    {
        self.pods.iter()
            .filter(|pod| pod.moves_taken != MovesTaken::Organized)
            .next()
            .is_none()
    }

    fn is_any_amphipod_at(&self, pos: Point) -> bool
    {
        self.pods.iter()
            .filter(|pod| pod.pos == pos)
            .next()
            .is_some()
    }

    fn steps_to_move_from_to(&self, start: Point, end: Point) -> Option<usize>
    {
        let mut cur = start;
        let mut num_steps = 0;

        // First, move into the corridor

        while cur.y > 1
        {
            cur.y -= 1;
            num_steps += 1;

            if self.is_any_amphipod_at(cur)
            {
                return None;
            }
        }

        // Move along the corridor
        
        while cur.x != end.x
        {
            cur.x += (end.x - cur.x).signum();
            num_steps += 1;

            if self.is_any_amphipod_at(cur)
            {
                return None;
            }
        }

        // Move into the room

        while cur.y != end.y
        {
            cur.y += 1;
            num_steps += 1;

            if self.is_any_amphipod_at(cur)
            {
                return None;
            }
        }

        Some(num_steps)
    }

    fn cur_bottom_target_pos_for_room(&self, kind: Kind) -> Option<Point>
    {
        let x = kind.target_room_x();
        let mut y = self.max_y;

        while y >= 2
        {
            let pos = Point::new(x, y);

            if let Some(pod) = self.pods.iter().filter(|pod| pod.pos == pos).next()
            {
                if pod.kind == kind
                {
                    // There is an organized pod already - try
                    // the next space up
                    assert!(pod.moves_taken == MovesTaken::Organized);
                    y -= 1;
                    continue;
                }
                else
                {
                    // There is a non-organised 'pod in
                    // this room - we're not allowed to
                    // move organized 'pods into it until
                    // this one is removed
                    return None;
                }
            }
            else
            {
                // This point is empty - move here
                return Some(pos)
            }
        }
        None
    }

    fn next_moves(&self) -> Vec<(Burrow, usize)>
    {
        let mut result = Vec::new();

        for i in 0..self.pods.len()
        {
            if self.pods[i].moves_taken == MovesTaken::NotMoved
            {
                // Try moving to the corridor positions

                for new_x in [1, 2, 4, 6, 8, 10, 11]
                {
                    let new_pos = Point::new(new_x, 1);

                    if let Some(steps) = self.steps_to_move_from_to(self.pods[i].pos, new_pos)
                    {
                        let mut new_burrow = self.clone();
                        new_burrow.pods[i].pos = new_pos;
                        new_burrow.pods[i].moves_taken = MovesTaken::InCoridor;

                        result.push((new_burrow, steps * self.pods[i].kind.cost_per_step()));
                    }
                }
            }

            if (self.pods[i].moves_taken == MovesTaken::NotMoved)
                || (self.pods[i].moves_taken == MovesTaken::InCoridor)
            {
                // Try moving to the target room positions

                if let Some(new_pos) = self.cur_bottom_target_pos_for_room(self.pods[i].kind)
                {
                    if let Some(steps) = self.steps_to_move_from_to(self.pods[i].pos, new_pos)
                    {
                        let mut new_burrow = self.clone();
                        new_burrow.pods[i].pos = new_pos;
                        new_burrow.pods[i].moves_taken = MovesTaken::Organized;

                        result.push((new_burrow, steps * self.pods[i].kind.cost_per_step()));
                    }
                }
            }
        }

        result
    }

    fn huristic_cost_to_fully_organised(&self) -> usize
    {
        let mut result = 0;

        for pod in self.pods.iter()
        {
            if pod.moves_taken != MovesTaken::Organized
            {
                let steps =
                    ((pod.pos.y - 1).abs() as usize)
                    + ((pod.pos.x - pod.kind.target_room_x()).abs() as usize)
                    + 1;

                result += steps + pod.kind.cost_per_step();
            }
        }

        result
    }
}

fn solve(input: &str, part_2: bool) -> usize
{
    let start = Burrow::new(input, part_2);

    let (_, cost) = pathfinding::directed::astar::astar(
        &start,
        |burrow| burrow.next_moves(),
        |burrow| burrow.huristic_cost_to_fully_organised(),
        |burrow| burrow.is_fully_organised())
    .unwrap();

    cost
}

fn part_1(input: &str) -> usize
{
    solve(input, false)
}

fn part_2(input: &str) -> usize
{
    solve(input, true)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(23)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 12521,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 15516,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 44169,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 45272,
        })
}
