use std::str::FromStr;
use std::collections::HashMap;
use itertools::*;
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

#[derive(Debug, Clone, Copy)]
enum CompareOp
{
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone, Copy)]
struct Comparison
{
    rating_index: usize,
    op: CompareOp,
    val: i64,
}

impl FromStr for Comparison
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let (access_ch, op_ch, val) = scan(s)
            .take(1).parse()
            .take(1).parse()
            .remaining().parse();

        let rating_index = match access_ch
        {
            'x' => 0,
            'm' => 1,
            'a' => 2,
            's' => 3,
            _ => unreachable!(),
        };

        let op = match op_ch
        {
            '>' => CompareOp::GreaterThan,
            '<' => CompareOp::LessThan,
            _ => unreachable!(),
        };

        Ok(Comparison{ rating_index, op, val })
    }
}

#[derive(Debug, Clone)]
enum Action
{
    Accept,
    Reject,
    Move(String),
}

impl FromStr for Action
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        if s == "A"
        {
            Ok(Action::Accept)
        }
        else if s == "R"
        {
            Ok(Action::Reject)
        }
        else
        {
            Ok(Action::Move(s.to_string()))
        }
    }
}

#[derive(Debug)]
struct Rule
{
    comp: Option<Comparison>,
    action: Action,
}

impl FromStr for Rule
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let parts = s.split(':').collect_vec();

        if parts.len() == 1
        {
            Ok(Rule{ comp: None, action: parts[0].parse().unwrap() })
        }
        else if parts.len() == 2
        {
            Ok(Rule
            {
                comp: Some(parts[0].parse().unwrap()),
                action: parts[1].parse().unwrap(),
            })
        }
        else
        {
            unreachable!();
        }
    }
}

#[derive(Debug)]
struct Workflow
{
    name: String,
    rules: Vec<Rule>,
}

impl FromStr for Workflow
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let (name, rules,) = scan(s)
            .until("{")
            .parse()
            .until("}").parse_vec(",")
            .remaining().ignore();

        Ok(Workflow { name, rules })
    }
}

struct Part
{
    ratings: [i64;4],
}

impl FromStr for Part
{
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err>
    {
        let (x, m, a, s) = scan(s)
            .skip_str("{x=")
            .take_digits().parse()
            .skip_str(",m=")
            .take_digits().parse()
            .skip_str(",a=")
            .take_digits().parse()
            .skip_str(",s=")
            .take_digits().parse()
            .remaining().ignore();
        Ok(Part{ ratings: [x, m, a, s] })
    }
}

impl Part
{
    fn attribute_sum(&self) -> i64
    {
        self.ratings.iter().sum()
    }

    fn is_accepted(&self, workflows: &HashMap<String, Workflow>) -> bool
    {
        let mut cur_workflow_name = "in".to_string();
        loop
        {
            let workflow = workflows.get(&cur_workflow_name).unwrap();
            match self.process_rules(&workflow.rules)
            {
                Action::Accept => return true,
                Action::Reject => return false,
                Action::Move(target) => cur_workflow_name = target,
            }
        }
    }

    fn process_rules(&self, rules: &Vec<Rule>) -> Action
    {
        for r in rules.iter()
        {
            match &r.comp
            {
                Some(comp) =>
                {
                    let part_val = self.ratings[comp.rating_index];
                    let cond = match comp.op
                    {
                        CompareOp::GreaterThan => part_val > comp.val,
                        CompareOp::LessThan => part_val < comp.val,
                    };
                    if cond
                    {
                        return r.action.clone();
                    }
                },
                None =>
                {
                    return r.action.clone();
                },
            };
        }
        unreachable!();
    }
}

fn split_rating_ranges(ratings: Vec<RangeSet<i64>>, comp: Option<Comparison>) -> (Option<Vec<RangeSet<i64>>>, Option<Vec<RangeSet<i64>>>)
{
    match comp
    {
        None =>
        {
            return (Some(ratings), None);
        },
        Some(comp) =>
        {
            let mut if_true = ratings.clone();
            let mut if_false = ratings.clone();

            match comp.op
            {
                CompareOp::GreaterThan =>
                {
                    // rating > comp.val : remove smaller from if_true, larger from if_false
                    if_true[comp.rating_index].remove_range(RangeInc::new_range(0, comp.val));
                    if_false[comp.rating_index].remove_range(RangeInc::new_range(comp.val + 1, 4001));
                },
                CompareOp::LessThan =>
                {
                    // rating < comp.val : remove larger from if_true, smaller from if_false
                    if_true[comp.rating_index].remove_range(RangeInc::new_range(comp.val, 4001));
                    if_false[comp.rating_index].remove_range(RangeInc::new_range(0, comp.val - 1));
                },
            };

            let min_if_true = if_true.iter().map(|r| r.count()).min().unwrap();
            let min_if_false = if_false.iter().map(|r| r.count()).min().unwrap();

            let if_true = if min_if_true == 0 { None } else { Some(if_true) };
            let if_false = if min_if_false == 0 { None } else { Some(if_false) };

            (if_true, if_false)
        },
    }
}

fn count_distinct_for_rule(ratings: Vec<RangeSet<i64>>, r_index: usize, rules: &Vec<Rule>, workflows: &HashMap<String, Workflow>) -> i64
{
    let (opt_if_true, opt_if_false) = split_rating_ranges(ratings, rules[r_index].comp);

    let mut result = 0;

    if let Some(if_true) = opt_if_true
    {
        match &rules[r_index].action
        {
            Action::Accept =>
            {
                result += if_true.iter().map(|r| r.count()).product::<i64>();
            },
            Action::Reject =>
            {
                // Ignore
            },
            Action::Move(target) =>
            {
                result += count_distinct_for_workflow(if_true, target, workflows)
            }
        }
    }

    if let Some(if_false) = opt_if_false
    {
        result += count_distinct_for_rule(if_false, r_index + 1, rules, workflows);
    }

    result
}

fn count_distinct_for_workflow(ratings: Vec<RangeSet<i64>>, workflow_name: &String, workflows: &HashMap<String, Workflow>) -> i64
{
    if ratings.iter().map(|r| r.count()).min().unwrap() == 0
    {
        // At least one of the ratings has reached an empty range -
        // the product will be zero. No more work required.
        return 0;
    }

    let workflow = workflows.get(workflow_name).unwrap();

    count_distinct_for_rule(ratings, 0, &workflow.rules, workflows)
}

fn count_distinct_accepted_combinations(workflows: &HashMap<String, Workflow>) -> i64
{
    let range = RangeSet::new_from_range(RangeInc::new_range(1i64, 4000i64));
    let initial_ratings = vec![range.clone(), range.clone(), range.clone(), range.clone()];

    count_distinct_for_workflow(initial_ratings, &"in".to_string(), workflows)
}

fn parse_input(input: &str) -> (HashMap<String, Workflow>, Vec<Part>)
{
    let groups = input_to_groups(input);
    let workflows = groups[0].iter().map(|l| l.parse().unwrap()).map(|w: Workflow| (w.name.clone(), w)).collect();
    let parts = groups[1].iter().map(|l| l.parse().unwrap()).collect();

    (workflows, parts)
}

fn part_1(input: &str) -> i64
{
    let (workflows, parts) = parse_input(input);

    parts.into_iter()
        .filter(|p| p.is_accepted(&workflows))
        .map(|p| p.attribute_sum())
        .sum()
}

fn part_2(input: &str) -> i64
{
    let (workflows, _) = parse_input(input);
    count_distinct_accepted_combinations(&workflows)
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(19)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 19114,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 487623,
        })
        .example(|| Answer {
            calculated: part_2(EXAMPLE),
            expected: 167409079868000i64,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: 113550238315130i64,
        })
}
