use crate::support::*;
use itertools::*;

const EXAMPLE: &str = include_str!("example.txt");
const INPUT: &str = include_str!("input.txt");

struct SourceDestEntry
{
    source_range: RangeInc<u64>,
    dest_start: u64,
}

struct SourceDestMap
{
    entries: Vec<SourceDestEntry>
}

impl SourceDestMap
{
    fn map_ranges(&self, ranges: &RangeSet<u64>) -> RangeSet<u64>
    {
        // First:
        // 1) Start with an empty result set
        // 2) Mark all inputs as unmapped

        let mut result = RangeSet::new();
        let mut unmapped = ranges.clone();

        // Now - for each entry in our mapping:
        // 1) Find the intersection with the unmapped entries,
        // 2) Remove these from the umapped entries
        // 3) Insert the mapped values into the result

        for entry in self.entries.iter()
        {
            let mut intersection = unmapped.clone();
            intersection.insersect_with_range(entry.source_range);

            for range in intersection.ranges()
            {
                let start_offset = range.start - entry.source_range.start;
                let end_offset = range.end - entry.source_range.start;

                unmapped.remove_range(range);
                result.insert_range(RangeInc::new_range(entry.dest_start + start_offset, entry.dest_start + end_offset));
            }
        }

        // Finally - any un-mapped entries remaining
        // pass straigh through

        for unmapped_range in unmapped.ranges()
        {
            result.insert_range(unmapped_range);
        }

        result
    }
}

struct Almanac
{
    seeds: RangeSet<u64>,
    maps: Vec<SourceDestMap>,
}

impl Almanac
{
    fn parse(input: &str, part_2: bool) -> Self
    {
        let parse_line = |l: &str|
        {
            let parts = l.split(" ").map(|p| p.parse::<u64>().unwrap()).collect_vec();
            assert!(parts.len() == 3);
            let source_range = RangeInc::new_range(parts[1], parts[1] + parts[2] - 1);
            let dest_start = parts[0];
            SourceDestEntry { source_range, dest_start }
        };

        let parse_map = |g: &Vec<String>|
        {
            let entries = g[1..].iter()
                .map(|l| parse_line(l))
                .collect_vec();
            SourceDestMap { entries }
        };

        let groups = input_to_groups(input);

        let seed_list = groups[0][0].split(" ").skip(1).map(|s| s.parse::<u64>().unwrap()).collect_vec();
        let maps = groups[1..].iter().map(|g| parse_map(g)).collect_vec();

        let seeds = if part_2
        {
            let mut seeds = RangeSet::new();
            for (start, len) in seed_list.into_iter().tuples()
            {
                seeds.insert_range(RangeInc::new_range(start, start + len - 1));
            }
            seeds
        }
        else // part_1
        {
            let mut seeds = RangeSet::new();
            for seed in seed_list.into_iter()
            {
                seeds.insert_value(seed)
            }
            seeds
        };

        Almanac { seeds, maps }
    }

    fn map_seeds(&self, seeds: RangeSet<u64>) -> RangeSet<u64>
    {
        let mut cur = seeds;
        //println!("{:?}", cur);
        for map in self.maps.iter()
        {
            cur = map.map_ranges(&cur);
            //println!("    => {:?}", cur);
        }
        cur
    }
}

fn part_1(input: &str) -> u64
{
    let almanac = Almanac::parse(input, false);

    almanac.map_seeds(almanac.seeds.clone()).values().next().unwrap()
}

fn part_2(input: &str) -> u64
{
    let almanac = Almanac::parse(input, true);

    almanac.map_seeds(almanac.seeds.clone()).values().next().unwrap()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(5)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 35,
        })
        .part_1(|| Answer {
            calculated: part_1(INPUT),
            expected: 579439039,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 46,
        })
        .part_2(|| Answer {
            calculated: part_2(INPUT),
            expected: 7873084,
        })
}
