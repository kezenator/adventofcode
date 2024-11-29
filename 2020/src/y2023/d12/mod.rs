use std::str::FromStr;
use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Spring
{
    Unknown,
    Damaged,
    Operational,
}

impl Spring
{
    fn to_char(&self) -> char
    {
        match self
        {
            Spring::Unknown => '?',
            Spring::Damaged =>'#',
            Spring::Operational => '.',
        }
    }

    fn from_char(ch: char) -> Self
    {
        match ch
        {
            '?' => Spring::Unknown,
            '#' => Spring::Damaged,
            '.' => Spring::Operational,
            _ => panic!(),
        }
    }

    fn could_be_damaged(&self) -> bool
    {
        *self == Spring::Unknown || *self == Spring::Damaged
    }

    fn could_be_operational(&self) -> bool
    {
        *self == Spring::Unknown || *self == Spring::Operational
    }
}

#[derive(Clone)]
struct Row
{
    springs: Vec<Spring>,
    damaged_counts: Vec<usize>,
}

impl FromStr for Row
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let parts = s.split(' ').collect_vec();
        Ok(Row
        {
            springs: parts[0].chars().map(|ch| Spring::from_char(ch)).collect_vec(),
            damaged_counts: parts[1].split(',').map(|c| c.parse().unwrap()).collect_vec(),
        })
    }
}

impl std::fmt::Debug for Row
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        write!(f, "{}", self.springs.iter().map(|s| s.to_char()).collect::<String>())?;
        write!(f, " ")?;
        write!(f, "{}", self.damaged_counts.iter().map(|c| c.to_string()).join(","))
    }
}

#[derive(Hash, PartialEq, Eq, Clone)]
struct MemorizeState<'a>
{
    springs: &'a [Spring],
    damaged_counts: &'a [usize],
    remaining_damage: Option<usize>,
}

impl<'a> std::fmt::Debug for MemorizeState<'a>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.write_str(&self.springs.iter().map(|s| s.to_char()).collect::<String>())?;
        f.write_str(" ")?;
        f.write_str(&self.damaged_counts.iter().map(|d| d.to_string()).join(","))?;

        if self.remaining_damage.is_some()
        {
            write!(f, " {:?}", self.remaining_damage)?;
        }

        Ok(())
    }
}

fn count_arrangements<'a>(springs: &'a[Spring], damaged_counts: &'a[usize]) -> usize
{
    let initial_state = MemorizeState
    {
        springs,
        damaged_counts,
        remaining_damage: None,
    };

    Memorized::new(&move |input: &MemorizeState<'a>, memorized| -> usize
    {
        if (input.springs.len() == 0) && (input.damaged_counts.len() == 0)
        {
            if input.remaining_damage.is_some()
                && (input.remaining_damage.unwrap() > 0)
            {
                // Still remaining damage - not valid
                return 0;
            }
            else
            {
                // All done! Valid arrangement
                return 1;
            }
        }
        else if input.springs.len() == 0
        {
            // There is more damage but no springs remaining -
            // not a valid arangement
            return 0;
        }
        else // there is a next spring to process
        {
            let next_spring = input.springs[0].clone();

            match input.remaining_damage
            {
                Some(remaining_damage) =>
                {
                    if remaining_damage == 0
                    {
                        if next_spring.could_be_operational()
                        {
                            // Next spring completes damaged run
                            return memorized.get(&MemorizeState
                            {
                                springs: &input.springs[1..],
                                damaged_counts: &input.damaged_counts,
                                remaining_damage: None,
                            });
                        }
                        else
                        {
                            // Additional damaged springs longer than this run
                            return 0;
                        }
                    }
                    else if !next_spring.could_be_damaged()
                    {
                        // Next spring is definately operational
                        return 0;
                    }
                    else
                    {
                        // Consume spring as damaged
                        return memorized.get(&MemorizeState {
                            springs: &input.springs[1..],
                            damaged_counts: &input.damaged_counts,
                            remaining_damage: Some(remaining_damage - 1),
                        });
                    }
                },
                None =>
                {
                    // We are skipping operational springs
                    // to get to the next damaged run
                    if input.damaged_counts.len() == 0
                    {
                        // No more damaged counts - we can only accept operational springs
                        if next_spring.could_be_operational()
                        {
                            return memorized.get(&MemorizeState {
                                springs: &input.springs[1..],
                                damaged_counts: &input.damaged_counts,
                                remaining_damage: None,
                            })
                        }
                        else // next spring is damaged - not valid
                        {
                            return 0;
                        }
                    }
                    else
                    {
                        // OK - we've got both a next spring and
                        // a next damaged count - there are two possibilities...

                        let next_damaged_count = input.damaged_counts[0];
                        assert!(next_damaged_count > 0);

                        let mut result = 0;

                        if next_spring.could_be_operational()
                        {
                            // Treat the spring as operational
                            result += memorized.get(&MemorizeState {
                                springs: &input.springs[1..],
                                damaged_counts: &input.damaged_counts,
                                remaining_damage: None,
                            });
                        }

                        if next_spring.could_be_damaged()
                        {
                            // Treat the spring as a new run of damaged springs
                            result += memorized.get(&MemorizeState {
                                springs: &input.springs[1..],
                                damaged_counts: &input.damaged_counts[1..],
                                remaining_damage: Some(next_damaged_count - 1),
                            });
                        }
                        return result;
                    }
                },
            }
        }
    }).get(&initial_state)
}

impl Row
{
    fn unfold(&self) -> Self
    {
        let mut springs = Vec::new();
        let mut damaged_counts = Vec::new();

        for i in 0..5
        {
            if i != 0
            {
                springs.push(Spring::Unknown);
            }
            springs.append(&mut self.springs.clone());
            damaged_counts.append(&mut self.damaged_counts.clone());
        }

        Row { springs, damaged_counts }
    }

    fn count_arrangements(& self) -> usize
    {
        count_arrangements(&self.springs, &self.damaged_counts)
    }

}

fn part_1(input: &str) -> usize
{
    let rows = input_to_lines_parsed::<Row>(input);
    rows.iter().map(|r| r.count_arrangements()).sum()
}

fn part_2(input: &str) -> usize
{
    let rows = input_to_lines_parsed::<Row>(input);
    let rows = rows.iter().map(|r| r.unfold()).collect_vec();
    rows.iter().map(|r| r.count_arrangements()).sum()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(12)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 21,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 7922,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 525152,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 18093821750095usize,
        })
}
