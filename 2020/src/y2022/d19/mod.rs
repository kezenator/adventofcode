use crate::support::*;
use itertools::*;
use std::collections::HashSet;
use pathfinding::directed::bfs::*;

const EXAMPLE: &str = include_str!("example.txt");

#[derive(Clone, Debug)]
struct State
{
    elapsed: i64,
    resources: [i64;4],
    robots: [i64;4],
}

impl State
{
    fn wait(&self, minutes: i64) -> Self
    {
        let mut result = self.clone();
        result.elapsed += minutes;
        for i in 0..4
        {
            result.resources[i] += minutes * result.robots[i];
        }
        result
    }

    fn manufacture(&self, bp: &Blueprint, robot: usize) -> Self
    {
        let mut result = self.clone();

        result.elapsed += 1;

        for i in 0..3
        {
            assert!(result.resources[i] >= bp.robot_costs_ore[robot][i]);
            result.resources[i] -= bp.robot_costs_ore[robot][i];
        }

        for i in 0..4
        {
            result.resources[i] += result.robots[i];
        }

        result.robots[robot] += 1;

        result
    }

    fn time_until_can_manufacture(&self, bp: &Blueprint, robot: usize) -> Option<i64>
    {
        let mut max_time_required = 0;
        for i in 0..3
        {
            if bp.robot_costs_ore[robot][i] > self.resources[i]
            {
                let needed_resources =
                    bp.robot_costs_ore[robot][i]
                    - self.resources[i];

                let num_robots = self.robots[i];

                if num_robots == 0
                {
                    // No robots yet to build this required resource
                    return None;
                }

                let time_for_this_robot = (needed_resources + num_robots - 1) / num_robots;

                max_time_required = max_time_required.max(time_for_this_robot);
            }
        }

        Some(max_time_required)
    }

    fn greedy_estimate_of_max_geodes(&self, bp: &Blueprint, max_minutes: i64) -> i64
    {
        // Run the simulation, but assume infinite ore.
        // This removes all choices about which types of robots
        // to build at each step.

        let mut cur_state = self.clone();

        while cur_state.elapsed < max_minutes
        {
            cur_state.elapsed += 1;

            // Simulate clay robots
            cur_state.resources[1] += cur_state.robots[1];
            cur_state.robots[1] += 1;

            // Simulate obsidian robots
            cur_state.resources[2] += cur_state.robots[2];
            cur_state.robots[2] += cur_state.resources[1] / bp.robot_costs_ore[2][1];
            cur_state.resources[1] = cur_state.resources[1] % bp.robot_costs_ore[2][1];

            // Simulate geode robots
            cur_state.resources[3] += cur_state.robots[3];
            cur_state.robots[3] += cur_state.resources[2] / bp.robot_costs_ore[3][2];
            cur_state.resources[2] = cur_state.resources[2] % bp.robot_costs_ore[3][2];
        }

        cur_state.resources[3]
    }

    fn next_states(&self, bp: &Blueprint, max_minutes: i64) -> Vec<State>
    {
        let mut result = Vec::new();

        // Try to build each robot - prioritize geode
        // robots first to find better solutions earlier to
        // allow more trees to be pruned

        for robot in [3, 2, 1, 0]
        {
            match self.time_until_can_manufacture(bp, robot)
            {
                None =>
                {
                    // Not enough other robots built yet to
                    // collect resources to build this robot
                },
                Some(time) =>
                {
                    if self.elapsed + time + 1 <= max_minutes
                    {
                        result.push(self.wait(time).manufacture(bp, robot));
                    }
                },
            }
        }

        if result.is_empty()
        {
            // Can't make any more robots in the time available:
            // just wait until the maximum minutes

            assert!(self.elapsed < max_minutes);
            result.push(self.wait(max_minutes - self.elapsed));
        }

        result
    }
}

struct Blueprint
{
    index: i64,
    robot_costs_ore: [[i64;4];4],
}

impl Blueprint
{
    fn parse(input: &str) -> Blueprint
    {
        let (index, ore_ore_cost, clay_ore_cost, obsidian_ore_cost, obsidian_clay_cost, geode_ore_cost, geode_obsidian_cost) = scan(input)
            .skip_str("Blueprint ")
            .until(": Each ore robot costs ").parse()
            .until(" ore. Each clay robot costs ").parse()
            .until(" ore. Each obsidian robot costs ").parse()
            .until(" ore and ").parse()
            .until(" clay. Each geode robot costs ").parse()
            .until(" ore and ").parse()
            .until(" obsidian.").parse()
            .remaining().ignore();

        let robot_costs_ore =
        [
            [ore_ore_cost, 0, 0, 0],
            [clay_ore_cost, 0, 0, 0],
            [obsidian_ore_cost, obsidian_clay_cost, 0, 0],
            [geode_ore_cost, 0, geode_obsidian_cost, 0],
        ];

        Blueprint { index, robot_costs_ore }
    }

    fn max_geodes(&self, minutes: i64) -> i64
    {
        let start = State
        {
            elapsed: 0,
            resources: [0, 0, 0, 0],
            robots: [1, 0, 0, 0],
        };

        let search_result = search(
            start,
            |s|
            {
                s.next_states(self, minutes)
            },
            |s|
            {
                s.elapsed == minutes
            },
            |a, b|
            {
                // Want most geodes - so
                // search in reverse order
                b.resources[3].cmp(&a.resources[3])
            },
            |state, best_solution|
            {
                let estimate = state.greedy_estimate_of_max_geodes(self, minutes);
                estimate > best_solution.resources[3]
            },
            SearchDebugLevel::None).unwrap();

        search_result.resources[3]
    }
}

fn part_1(input: &str) -> i64
{
    input_to_lines(input).into_iter()
        .map(|l| Blueprint::parse(&l))
        .map(|bp|
        {
            let max_generated = bp.max_geodes(24);
            bp.index * max_generated
        })
        .sum()
}

fn part_2(input: &str) -> i64
{
    input_to_lines(input).into_iter()
        .map(|l| Blueprint::parse(&l))
        .take(3)
        .map(|bp|
        {
            bp.max_geodes(32)
        })
        .product()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(19)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 33,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 1427,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 3472,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 4400,
        })
}
