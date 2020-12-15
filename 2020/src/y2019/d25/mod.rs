use std::str::FromStr;
use std::default::Default;
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use crate::support::*;
use crate::y2019::intcode::*;

const INPUT: &str = include_str!("input.txt");

struct ItemCombiner
{
    turns: VecDeque<NextTurn>,
}

impl ItemCombiner
{
    pub fn new(items: HashSet<String>, direction_to_pad: Direction) -> Self
    {
        // Top level algorithm:
        // 1) Try each combination of items on the pad
        //
        // For each combination:
        // 1) Take/drop items to reach combination
        // 2) Move to pad
        //
        // We start holding *all* items.
        //
        // Try combinations with a mid-range number of items first.
        // For example, for 5 items, try:
        // combinations of 3, combinations of 2 and 4, combinations of 1 and 5....

        let mid_cadinality = items.len() / 2;

        let mut cardinalities = (0..(items.len() + 1))
            .collect::<Vec<usize>>();

        cardinalities.sort_by(|a, b| (*a as isize - mid_cadinality as isize).abs().cmp(&(*b as isize - mid_cadinality as isize).abs()));

        let mut turns = VecDeque::new();
        let mut cur_items = items.clone();

        for cardi in cardinalities
        {
            for comb in items.clone().iter().combinations(cardi)
            {
                // Add and remove each item so the current items
                // match this combination

                for item in items.iter()
                {
                    if cur_items.contains(item)
                        && comb.iter().find(|&&i| i == item).is_none()
                    {
                        turns.push_back(NextTurn::Drop(item.clone()));
                        cur_items.remove(item);
                    }
                    else if !cur_items.contains(item)
                        && comb.iter().find(|&&i| i == item).is_some()
                    {
                        turns.push_back(NextTurn::Take(item.clone()));
                        cur_items.insert(item.clone());
                    }
                }

                // Try moding to the pad

                turns.push_back(NextTurn::Move(direction_to_pad));
            }
        }

        ItemCombiner
        {
            turns,
        }
    }

    pub fn next_turn(&mut self) -> NextTurn
    {
        self.turns.pop_front().unwrap()
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction
{
    North,
    South,
    East,
    West,
}

impl Direction
{
    pub fn opposite(&self) -> Self
    {
        match self
        {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }
}

impl ToString for Direction
{
    fn to_string(&self) -> String
    {
        match self
        {
            Direction::North => "north",
            Direction::South => "south",
            Direction::East => "east",
            Direction::West => "west",
        }.to_owned()
    }
}

impl FromStr for Direction
{
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        match s
        {
            "north" => Ok(Direction::North),
            "south" => Ok(Direction::South),
            "east" => Ok(Direction::East),
            "west" => Ok(Direction::West),
            _ => Err(format!("Unknown direction {:?}", s)),
        }
    }
}

enum NextTurn
{
    Move(Direction),
    Take(String),
    Drop(String),
    FoundPassword(String),
}

struct DecodedRoom
{
    name: String,
    description: String,
    directions: HashSet<Direction>,
    password: Option<String>,
    items: HashSet<String>,
}

impl DecodedRoom
{
    fn decode<I>(it: &mut itertools::PutBack<I>) -> Option<Self>
        where I: Iterator<Item=String>
    {
        let first = it.next().unwrap();
        if first == "Command?"
        {
            it.put_back(first);
            return None;
        }

        assert_eq!(first, String::new());
        assert_eq!(it.next(), Some(String::new()));
        assert_eq!(it.next(), Some(String::new()));

        let name = it.next().unwrap();
        let description = it.next().unwrap();

        assert_eq!(it.next(), Some(String::new()));
        
        let mut directions = HashSet::new();
        let mut password = None;
        let mut items = HashSet::new();

        assert_eq!(it.next(), Some("Doors here lead:".to_owned()));

        loop
        {
            let line = it.next().unwrap();

            if line.is_empty()
            {
                break;
            }
            directions.insert(line[2..].parse().unwrap());
        }

        let mut next = it.next().unwrap();

        if next.starts_with("A loud,")
        {
            let mut need_blank = true;

            if next.find("Droids on this ship are heavier than the detected value").is_some()
            {
                // Wrong weight - no password
            }
            else if next.find("Droids on this ship are lighter than the detected value").is_some()
            {
                // Wrong weight - no password
            }
            else if next.find("You may proceed.").is_some()
            {
                next = it.next().unwrap();
                assert_eq!(next, "Santa notices your small droid, looks puzzled for a moment, realizes what has happened, and radios your ship directly.");

                next = it.next().unwrap();
                assert!(next.starts_with("\"Oh, hello! You should be able to get in by typing"));
                assert!(next.ends_with(" on the keypad at the main airlock.\""));
                
                password = Some(next.chars().filter(|ch| ch.is_ascii_digit()).collect());
                need_blank = false;
            }
            else
            {
                unreachable!();
            }

            if need_blank
            {
                next = it.next().unwrap();
            }
        }

        if next.starts_with("Items")
        {
            loop
            {
                next = it.next().unwrap();

                if next.is_empty()
                {
                    break;
                }

                items.insert(next[2..].to_owned());
            }
        }
        else
        {
            it.put_back(next);
        }

        Some(DecodedRoom
        {
            name,
            description,
            directions,
            password,
            items,
        })
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
enum Location
{
    Known{ name: String },
    Unknown{ from: String, direction: Direction },
}

impl Location
{
    pub fn unwrap_known(&self) -> String
    {
        match self
        {
            Location::Known{name} => name.clone(),
            Location::Unknown{..} => unreachable!(),
        }
    }
}

struct VisitedLocation
{
    pub name: String,
    pub description: String,
    pub directions: HashMap<Direction, Location>,
    pub password: Option<String>,
    pub items: HashSet<String>,
}

impl Default for VisitedLocation
{
    fn default() -> Self
    {
        VisitedLocation
        {
            name: String::new(),
            description: String::new(),
            directions: HashMap::new(),
            password: None,
            items: HashSet::new(),
        }
    }
}

struct Droid
{
    comp: Intcode,
    cur_location: Option<Location>,
    visited_locations: HashMap<Location, VisitedLocation>,
    unvisited_locations: HashSet<Location>,
    collecting: bool,
    weight_moves: Option<ItemCombiner>,
    inventory: HashSet<String>,
}

impl Droid
{
    pub fn new() -> Self
    {
        Droid
        {
            comp: Intcode::new_from_input(INPUT),
            cur_location: None,
            visited_locations: HashMap::new(),
            unvisited_locations: HashSet::new(),
            collecting: true,
            weight_moves: None,
            inventory: HashSet::new(),
        }
    }

    fn run_until_output(&mut self) -> String
    {
        let _ = self.comp.run_until_halt_or_input_required();

        let result = self.comp.all_output().iter()
            .map(|i| (*i as u8) as char)
            .collect();

        //println!();
        //println!("OUTPUT");
        //println!("=======================");
        //println!("{}", result);

        result
    }

    fn input(&mut self, input: &str)
    {
        //println!();
        //println!("INPUT");
        //println!("=======================");
        //println!("{}", input);
        //std::thread::sleep_ms(100);

        for ch in input.chars()
        {
            self.comp.input(ch as i64);
        }
        self.comp.input(10);
    }

    fn direction_to(&self, target: &Location) -> (Direction, Location)
    {
        let path = pathfinding::directed::dijkstra::dijkstra::<(Direction, Location), usize, _, _, _>(
            &(Direction::North, self.cur_location.clone().unwrap()),
            |(_dir, location)|
            {
                let mut result = Vec::new();

                if let Location::Known{..} = location
                {
                    if let Some(visited) = self.visited_locations.get(location)
                    {
                        for (next_dir, next_loc) in visited.directions.iter()
                        {
                            result.push(((next_dir.clone(), next_loc.clone()), 1));
                        }
                    }
                }

                result
            },
            |(_dir, location)| *location == *target);

        assert!(path.is_some());

        let path = path.unwrap();
        assert!(path.0.len() >= 2);

        path.0[1].clone()
    }

    fn next_turn(&mut self) -> NextTurn
    {
        // First, if we're still collecting items and there
        // are any items in this room, then try and pick them up.
        // Note that there are some items that are not able to 
        // be picked up:
        //
        // "molten lava"         => it's too hot - you melt
        // "giant electromagnet" => it sticks to you and you can't move
        // "photons"             => it is suddenly completely dark and you are eaten by a Grue!
        // "escape pod"          => you're launched into space. Bye!
        // "infinite loop"       => ...........................................................

        if self.collecting
        {
            if let Some(cur_loc) = self.cur_location.clone()
            {
                if let Some(visited) = self.visited_locations.get(&cur_loc)
                {
                    for item in visited.items.iter()
                    {
                        if *item != "molten lava"
                            && *item != "giant electromagnet"
                            && *item != "photons"
                            && *item != "escape pod"
                            && *item != "infinite loop"
                        {
                            return NextTurn::Take(visited.items.iter().next().unwrap().clone());
                        }
                    }
                }
            }
        }

        // OK - no items in the current room - try and
        // move to the next unvisited location - so we collect
        // all known items

        if !self.unvisited_locations.is_empty()
        {
            let movement = self.direction_to(&self.unvisited_locations.iter().next().unwrap());

            if let Location::Unknown{..} = movement.1
            {
                // We've actually moved to a new unknown location - not
                // a previously visited known location.
                // Remove it from the list of unvisited locations.

                self.unvisited_locations.remove(&movement.1);
            }

            return NextTurn::Move(movement.0);
        }

        // OK - we've visited everywhere and picked everything up -
        // lets try and move to the "== Security Checkpoint =="
        // so we can start seeing which items we need to try
        // to get the weight correct

        if self.collecting == true
        {
            if let Some(cur_loc) = self.cur_location.clone()
            {
                let security_checkpoint = Location::Known{ name: "== Security Checkpoint ==".to_owned() };
                let pressure_sensitive_floor = Location::Known{ name: "== Pressure-Sensitive Floor ==".to_owned() };

                if cur_loc != security_checkpoint
                {
                    return NextTurn::Move(self.direction_to(&security_checkpoint).0);
                }

                // OK - we've finished collecting all items
                // and have moved to the security checkpoint.
                //
                // Now we need to start dropping/taking items
                // until we find the correct combination

                self.collecting = false;

                self.weight_moves = Some(ItemCombiner::new(
                    self.inventory.clone(),
                    self.direction_to(&pressure_sensitive_floor).0
                ));
            }
        }

        // If we've reached the pressure senstive pad and been given
        // a password, then we need to return it

        if let Some(cur_loc) = self.cur_location.clone()
        {
            if let Some(visited) = self.visited_locations.get(&cur_loc)
            {
                if let Some(password) = visited.password.clone()
                {
                    return NextTurn::FoundPassword(password);
                }
            }
        }

        // Finally, we have to keep trying different combinations
        // of items until we find the correct combination of
        // weights that triggers the pressure sensitive pad.

        if let Some(weight_moves) = &mut self.weight_moves
        {
            return weight_moves.next_turn();
        }

        // Oops! We're out of ideas about
        // what to do next.......

        unreachable!();
    }

    fn decode_move_outputs(&mut self, output: &str) -> (DecodedRoom, Option<DecodedRoom>)
    {
        let mut it = itertools::put_back(input_to_lines(output));

        let first = DecodedRoom::decode(&mut it).unwrap();

        if first.password.is_some()
        {
            // We've found the password - there is no second room,
            // and no "Command?" prompt

            return (first, None);
        }

        let second = DecodedRoom::decode(&mut it);

        assert_eq!(it.next(), Some("Command?".to_owned()));

        (first, second)
    }

    fn process_one_move_output(&mut self, direction: Direction, decoded: DecodedRoom)
    {
        // Create a new know location

        let location = Location::Known{ name: decoded.name.clone() };

        // Save the decoded details

        let visited = self.visited_locations.entry(location.clone()).or_default();

        visited.name = decoded.name;
        visited.description = decoded.description;
        visited.password = decoded.password;
        visited.items = decoded.items;

        // If there are any decoded directions that are not stored
        // in the visited structure, then add them and also add them
        // as new unvisited locations

        for dir in decoded.directions
        {
            if !visited.directions.contains_key(&dir)
            {
                visited.directions.insert(dir.clone(), Location::Unknown{ from: visited.name.clone(), direction: dir.clone(), });
                self.unvisited_locations.insert(Location::Unknown{ from: visited.name.clone(), direction: dir.clone(), });
            }
        }

        // If we already had a current location, then update
        // all of the links between these two locations, and ensure that
        // the directions from these two known locations are no longer
        // in the unvisited list

        if let Some(prev_loc) = self.cur_location.clone()
        {
            let next_loc = location.clone();

            self.visited_locations.get_mut(&prev_loc).unwrap().directions.insert(direction, next_loc.clone());
            self.visited_locations.get_mut(&next_loc).unwrap().directions.insert(direction.opposite(), prev_loc.clone());

            self.unvisited_locations.remove(&Location::Unknown{ from: prev_loc.unwrap_known(), direction: direction });
            self.unvisited_locations.remove(&Location::Unknown{ from: next_loc.unwrap_known(), direction: direction.opposite() });
        }

        // Save the updated current location

        self.cur_location = Some(location);
    }

    fn process_move_outputs(&mut self, dir: Direction, output: &str)
    {
        let (first, second) = self.decode_move_outputs(output);

        self.process_one_move_output(dir, first);

        if let Some(second) = second
        {
            self.process_one_move_output(dir.opposite(), second);
        }
    }

    pub fn run_part_1(&mut self) -> String
    {
        // First - process the first location output by the game

        {
            assert!(self.cur_location.is_none());

            let output = self.run_until_output();

            let (first, second) = self.decode_move_outputs(&output);

            assert!(second.is_none());

            // The first direction doesn't matter, as there is no
            // current location, and no forced second move.

            self.process_one_move_output(Direction::North, first);
        }

        // Now, input each turn and process the relevant output

        loop
        {
            match self.next_turn()
            {
                NextTurn::Move(direction) =>
                {
                    self.input(&direction.to_string());

                    let output = self.run_until_output();

                    self.process_move_outputs(direction, &output);
                },
                NextTurn::Take(item) =>
                {
                    // Take the item

                    self.input(&format!("take {}", item));

                    // Verify the output

                    let output = self.run_until_output();

                    assert_eq!(output,
                        format!("\nYou take the {}.\n\nCommand?\n", item));

                    // Mark that it's no longer in the current room, and instead
                    // is in the inventory.

                    let cur_loc = self.cur_location.clone().unwrap();
                    let visited = self.visited_locations.get_mut(&cur_loc).unwrap();

                    visited.items.remove(&item);
                    self.inventory.insert(item);
                },
                NextTurn::Drop(item) =>
                {
                    // Drop the item

                    self.input(&format!("drop {}", item));

                    // Verify the output

                    let output = self.run_until_output();

                    assert_eq!(output,
                        format!("\nYou drop the {}.\n\nCommand?\n", item));

                    // Mark that it's added to the current room, and removed
                    // from the inventory

                    let cur_loc = self.cur_location.clone().unwrap();
                    let visited = self.visited_locations.get_mut(&cur_loc).unwrap();

                    self.inventory.remove(&item);
                    visited.items.insert(item);
                },
                NextTurn::FoundPassword(password) =>
                {
                    return password;
                },
            }
        }
    }
}

fn part_1() -> String
{
    let mut droid = Droid::new();
    droid.run_part_1()
}

fn part_2() -> usize
{
    0
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(25)
        .part_1(|| Answer { calculated: part_1(), expected: "805306888", })
        .part_2(|| Answer { calculated: part_2(), expected: 0, })
}
