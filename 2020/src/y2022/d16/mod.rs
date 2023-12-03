use crate::support::*;
use itertools::*;
use std::collections::HashMap;
use pathfinding::directed::astar::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

struct Location
{
    name: String,
    flow_rate: i64,
    dests: Vec<String>,
}

impl std::str::FromStr for Location
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let remove_plural = s.replace("tunnels", "tunnel");
        let remove_plural = remove_plural.replace("valves", "valve");
        let remove_plural = remove_plural.replace("leads", "lead");
        let (name, flow_rate, dests) = scan(&remove_plural)
            .skip_str("Valve ")
            .until(" has flow rate=").parse()
            .until("; tunnel lead to valve ").parse()
            .remaining().parse::<String>();

        let dests = dests.split(", ").map(|s| s.to_string()).collect_vec();

        Ok(Location{ name, flow_rate, dests })
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct MemorizedInput
{
    players: Vec<(String, i64)>,
    closed_valves: Vec<String>,
}

struct Map
{
    locations: HashMap<String, Location>,
    distances: HashMap<(String, String), i64>,
    valves: Vec<String>,
}

impl Map
{
    fn new(input: &str) -> Self
    {
        let locations =  input_to_lines_parsed::<Location>(input)
            .into_iter()
            .map(|l| (l.name.clone(), l))
            .collect::<HashMap<_,_>>();

        let mut distances = HashMap::new();

        let important_node_names = locations
            .values()
            .filter(|l| l.name == "AA" || l.flow_rate > 0)
            .map(|l| l.name.clone())
            .collect_vec();

        for i in 0..(important_node_names.len() - 1)
        {
            let i_name = &important_node_names[i];

            for j in 0..important_node_names.len()
            {
                let j_name = &important_node_names[j];

                let distance = astar(
                    i_name,
                    |loc|
                    {
                        locations.get(loc).unwrap().dests
                            .iter()
                            .map(|d| (d.clone(), 1))
                    },
                    |_| 0,
                    |loc| *loc == *j_name).unwrap().1;

                distances.insert((i_name.clone(), j_name.clone()), distance);
                distances.insert((j_name.clone(), i_name.clone()), distance);
            }
        }

        let valves = locations
            .values()
            .filter(|l| l.flow_rate > 0)
            .map(|l| l.name.clone())
            .sorted()
            .collect_vec();

        Map { locations, distances, valves }
    }

    fn move_single_player(&self, input: &mut (MemorizedInput, i64), player_index: usize, valve_to_open: &String)
    {
        //assert!(player_index < input.0.players.len());
        //assert!(input.0.closed_valves.contains(valve_to_open));

        let player_location = &input.0.players[player_index].0;
        let player_time_remaining = input.0.players[player_index].1;

        let dist = self.distances
            .get(&(player_location.clone(), valve_to_open.clone()))
            .unwrap();

        if (dist + 1) >= player_time_remaining
        {
            // Can't get to an open this valve in time - there is
            // no additional pressure we can release
        }
        else
        {
            // We can get to this valve and open it.
            // Then see if we can open any more

            let next_time_remaining = player_time_remaining - (dist + 1);

            let released_by_this_valve =
                next_time_remaining
                * self.locations.get(valve_to_open).unwrap().flow_rate;

            let valve_index = input.0.closed_valves
                .iter()
                .enumerate()
                .filter(|(_,v)| **v == *valve_to_open)
                .map(|(i,_)| i)
                .next()
                .unwrap();

            input.0.players[player_index] = (valve_to_open.clone(), next_time_remaining);
            input.0.closed_valves.remove(valve_index);
            input.1 += released_by_this_valve;
        }
    }

    fn move_players(&self, input: &MemorizedInput) -> Vec<(MemorizedInput, i64)>
    {
        if input.players.len() > input.closed_valves.len()
        {
            // There are more players than closed valves -
            // just try moving a single player - the one with
            // the most time remaining

            let max_player_time_remaining = input.players
                .iter()
                .map(|(_,t)| *t)
                .max()
                .unwrap();

            let player_index = input.players
                .iter()
                .enumerate()
                .filter(|(_,(_,t))| *t == max_player_time_remaining)
                .map(|(i,_)| i)
                .next()
                .unwrap();

            input.closed_valves
                .iter()
                .map(|valve_to_open|
                {
                    let mut mv = (input.clone(), 0);
                    self.move_single_player(
                        &mut mv,
                        player_index,
                        valve_to_open);
                    mv
                })
                .filter(|(_, direct_release)| *direct_release > 0)
                .collect()
        }
        else
        {
            // To reduce the memory use in the memorization system,
            // move all players at the same time. This means we need to
            // move the players based on the PERMUTATIONS of closed valves.
            // But a major optimization - at the start (when all players are identical)
            // it doesn't actually matter who moves on which path - so we only
            // need to test the COMBINATIONS.

            let all_players_equal = input.players
                .iter()
                .tuple_windows()
                .all(|(a, b)| *a == *b);

            let permutations = if all_players_equal
            {
                input.closed_valves.iter().combinations(input.players.len()).collect_vec()
            }
            else
            {
                input.closed_valves.iter().permutations(input.players.len()).collect_vec()
            };

            permutations
                .into_iter()
                .map(|valves_to_open|
                {
                    let mut mv = (input.clone(), 0);
                    for i in 0..valves_to_open.len()
                    {
                        self.move_single_player(
                            &mut mv,
                            i,
                            valves_to_open[i]);
                    }
                    mv
                })
                .filter(|(_, direct_release)| *direct_release > 0)
                .collect()
        }
    }

    fn most_pressure_released_from_loc_to_closed_valves_in_time(
        &self,
        input: &MemorizedInput,
        recurse: &Memorized<MemorizedInput, i64>) -> i64
    {
        assert!(!input.players.is_empty());
        assert!(!input.closed_valves.is_empty());

        // 1) Get a vector of combinations of player moves:
        //    Each move has the moved input state, plus the direct extra
        //    pressure released by the valves opened in that step.
        // 2) Iterate over them, and if there are still closed
        //    valves, then RECURSE to add on the additional pressure
        //    that can be released by opening them.
        // 3) Find the maximum - or zero if there are no more moves possible

        self.move_players(input)
            .into_iter()
            .map(|(new_input, direct_pressure_release)|
            {
                if new_input.closed_valves.is_empty()
                {
                    direct_pressure_release
                }
                else
                {
                    direct_pressure_release
                        + recurse.get(&new_input)
                }
            })
            .max()
            .unwrap_or(0)
    }

    fn most_pressure_released(&self, num_players: usize, time_to_work: i64) -> i64
    {
        Memorized::new(
            &|input, recurse|
            {
                self.most_pressure_released_from_loc_to_closed_valves_in_time(input, recurse)
            })
            //.debug(true)
            .get(&MemorizedInput
            {
                players: vec![("AA".to_string(), time_to_work); num_players],
                closed_valves: self.valves.clone(),
            })
    }
}

fn most_pressure_released(input: &str, num_players: usize, time_to_work: i64) -> i64
{
    Map::new(input)
        .most_pressure_released(num_players, time_to_work)
}

fn part_1(input: &str) -> i64
{
    most_pressure_released(input, 1, 30)
}

fn part_2(input: &str) -> i64
{
    most_pressure_released(input, 2, 26)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(16)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 1651,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 1754,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 1707,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 2474,
        })
}
