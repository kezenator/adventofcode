use std::collections::HashMap;
use itertools::Itertools;
use regex::Regex;
use crate::support::*;

const EXAMPLE_1: &str = "0: 4 1 5\n1: 2 3 | 3 2\n2: 4 4 | 5 5\n3: 4 5 | 5 4\n4: \"a\"\n5: \"b\"\n\nababbb\nbababa\nabbbab\naaabbb\naaaabbb";
const EXAMPLE_2: &str = r#"42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: "a"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: "b"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba"#;

const INPUT: &str = include_str!("input.txt");

#[derive(Clone, Debug, PartialEq, Eq)]
enum Rule
{
    Match(char),
    Seq(Vec<u64>),
    SeqOrSeq(Vec<u64>, Vec<u64>),
}

impl Rule
{
    pub fn parse(line: &str) -> (u64, Self)
    {
        let (num, rest) = scan(line)
            .until(": ").parse::<u64>()
            .remaining().parse::<String>();

        let rule = if rest.chars().next() == Some('"')
        {
            assert_eq!(rest.chars().count(), 3);
            
            Rule::Match(rest.chars().nth(1).unwrap())
        }
        else if rest.find(" | ").is_some()
        {
            let (a, b) = scan(&rest)
                .until(" | ").parse_vec::<u64>(" ")
                .remaining().parse_vec::<u64>(" ");

            Rule::SeqOrSeq(a, b)
        }
        else
        {
            Rule::Seq(scan(&rest).remaining().parse_vec::<u64>(" ").0)
        };

        (num, rule)
    }
}

struct Input
{
    rules: HashMap<u64, Rule>,
    messages: Vec<String>,
}

impl Input
{
    pub fn parse(input: &str) -> Self
    {
        let groups = input_to_groups(input);

        let rules = groups[0].iter().map(|l| Rule::parse(l)).collect();
        let messages = groups[1].iter().cloned().collect();

        Self { rules, messages }
    }

    fn rule_to_regex(&self, rule: u64) -> String
    {
        match self.rules.get(&rule).unwrap()
        {
            Rule::Match(ch) => format!("{}", ch),
            Rule::Seq(seq) => format!("{}", seq.iter().map(|r| self.rule_to_regex(*r)).join("")),
            Rule::SeqOrSeq(a, b) => format!("({}|{})",
                a.iter().map(|r| self.rule_to_regex(*r)).join(""),
                b.iter().map(|r| self.rule_to_regex(*r)).join("")),
        }
    }

    fn min_len_for_rule(&self, rule: u64) -> usize
    {
        match self.rules.get(&rule).unwrap()
        {
            Rule::Match(_ch) => 1,
            Rule::Seq(seq) => seq.iter().map(|r| self.min_len_for_rule(*r)).sum(),
            Rule::SeqOrSeq(a, b) => std::cmp::min(
                a.iter().map(|r| self.min_len_for_rule(*r)).sum(),
                b.iter().map(|r| self.min_len_for_rule(*r)).sum()),
        }
    }

    fn max_len_for_rule(&self, rule: u64) -> usize
    {
        match self.rules.get(&rule).unwrap()
        {
            Rule::Match(_ch) => 1,
            Rule::Seq(seq) => seq.iter().map(|r| self.max_len_for_rule(*r)).sum(),
            Rule::SeqOrSeq(a, b) => std::cmp::max(
                a.iter().map(|r| self.max_len_for_rule(*r)).sum(),
                b.iter().map(|r| self.max_len_for_rule(*r)).sum()),
        }
    }
}

fn part_1(input: &str) -> usize
{
    let input = Input::parse(input);

    let re = Regex::new(&format!("^{}$", input.rule_to_regex(0))).unwrap();

    input.messages.iter()
        .filter(|s| re.is_match(s))
        .count()
}

fn part_2(input: &str) -> usize
{
    let input = Input::parse(input);

    // For the provided inputs, we have the following rules:
    //
    // 0: 8 11
    // 8: 42
    // 11: 42 31
    //
    // We want to replace this with:
    // 0: 8 11 (same)
    // 8: 42 | 42 8
    // 11: 42 31 | 42 11 31
    //
    // This means rule 0 is kind of
    // 0: 42{2,} 31{1,}
    // and the number of 42's is at least one larger than the number of 31s

    // First, check the input matches these assumptions

    assert_eq!(input.rules.get(&0).cloned(), Some(Rule::Seq(vec![8, 11])));
    assert_eq!(input.rules.get(&8).cloned(), Some(Rule::Seq(vec![42])));
    assert_eq!(input.rules.get(&11).cloned(), Some(Rule::Seq(vec![42, 31])));

    // Get the regexs for these sub bits, and their minimum and maximum length

    let min_len_42 = input.min_len_for_rule(42);
    let min_len_31 = input.min_len_for_rule(31);

    let max_len_42 = input.max_len_for_rule(42);
    let max_len_31 = input.max_len_for_rule(31);

    let regex_str_42 = input.rule_to_regex(42);
    let regex_str_31 = input.rule_to_regex(31);

    // A function that checks if a string matches
    // the modified pattern

    let matches_str = |s: &str| -> bool
    {
        // Iterate for different lengths of patten 42/31
        // until we find combinations that meet the critierias
        // 1) num_42 > num_31
        // 2) the min/max length of the matched string matches
        //    the input string length

        let input_len = s.len();

        for num_31 in 1..((input_len / min_len_31) + 2)
        {
            for num_42 in (num_31 + 1)..((input_len / min_len_42) + 2)
            {
                let min_trial_len = (num_31 * min_len_31) + (num_42 * min_len_42);
                let max_trial_len = (num_31 * max_len_31) + (num_42 * max_len_42);

                if min_trial_len <= input_len && max_trial_len >= input_len
                {
                    // Ok - construct a regexp that matches this particular
                    // number of 42/31 patterns and see if it matches the input string

                    let re = Regex::new(&format!(
                        "^({}){}{}{}({}){}{}{}$",
                        regex_str_42, '{', num_42, '}',
                        regex_str_31, '{', num_31, '}',
                    )).unwrap();

                    if re.is_match(s)
                    {
                        return true;
                    }
                }
            }
        }
        
        // There is no combination of patterns that
        // could meet this input string

        false
    };

    input.messages.iter()
        .filter(|s| matches_str(s))
        .count()
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(19)
        .example(|| Answer { calculated: part_1(EXAMPLE_1), expected: 2, })
        .part_1(|| Answer { calculated: part_1(INPUT), expected: 173, })
        .example(|| Answer { calculated: part_2(EXAMPLE_2), expected: 12, })
        .part_2(|| Answer { calculated: part_2(INPUT), expected: 367, })
}
