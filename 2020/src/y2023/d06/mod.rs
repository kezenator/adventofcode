use crate::support::{*, num::diophantine::Quadratic};
use itertools::*;

const EXAMPLE: &str = "Time:      7  15   30\nDistance:  9  40  200";
const INPUT: &str = "Time:        61     70     90     66\nDistance:   643   1184   1362   1041";

struct Race
{
    time: i64,
    distance: i64,
}

impl Race
{
    fn times_to_beat_record(&self) -> RangeSet<i64>
    {
        // Distance = held * (time - held)
        // (held speed aquired, and that distance traveled for each of the (time - held) remaining seconds)
        // => d = -x*x + time*x
        // But - we want ways that are longer than the distance...
        // So we can take positive values of:
        // => extra = -x*x + time*x - distance

        let equation = Quadratic::new(-1, self.time, -self.distance);
        let solution = equation.solve();
        solution.positive
    }
}

fn parse_1(input: &str) -> Vec<Race>
{
    let lines = input_to_lines(input);
    let times = lines[0].split_ascii_whitespace().skip(1).map(|p| p.parse().unwrap()).collect_vec();
    let distances = lines[1].split_ascii_whitespace().skip(1).map(|p| p.parse().unwrap()).collect_vec();

    times.iter().zip(distances.iter())
        .map(|(&time, &distance)| Race { time, distance })
        .collect_vec()
}

fn parse_2(input: &str) -> Race
{
    let lines = input_to_lines(input);
    let time = lines[0].chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap();
    let distance = lines[1].chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap();
    Race { time, distance }
}

fn part_1(input: &str) -> i64
{
    let races = parse_1(input);

    races.iter()
        .map(|r| r.times_to_beat_record().count())
        .product()
}

fn part_2(input: &str) -> i64
{
    let race = parse_2(input);
    race.times_to_beat_record().count()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(6)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 288,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 293046,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 71503,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 35150181,
        })
}
