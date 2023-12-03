pub use std::cmp::Ordering;
use std::collections::VecDeque;
use std::fmt::Debug;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub enum SearchDebugLevel
{
    None,
    FinalStats,
    Solutions,
    Everything,
}

pub fn search<S, FN, I, FS, FC, FR>(
    initial: S,
    neighbours: FN,
    is_success: FS,
    compare_successes: FC,
    can_state_reach_better_than_solution: FR,
    debug: SearchDebugLevel) -> Option<S>
    where
    S: Debug,
    FN: Fn(&S) -> I,
    I: IntoIterator<Item = S>,
    FS: Fn(&S) -> bool,
    FC: Fn(&S, &S) -> Ordering,
    FR: Fn(&S, &S) -> bool,
{
    let mut best_solution = None;
    let mut generation = 0;
    let mut total_generated = 0u64;
    let mut rejected_solutions = 0u64;
    let mut total_abandoned = 0u64;

    let mut to_search = VecDeque::new();
    to_search.push_back(StateCompare{
        cmp: &compare_successes,
        generation: generation,
        state: initial
    });

    while !to_search.is_empty()
    {
        let prev = to_search.pop_front().unwrap();

        if prev.generation >= generation
        {
            generation = prev.generation + 1;

            if debug >= SearchDebugLevel::Solutions
            {
                println!("Starting generation {} with {} generated, {} abandoned, {} queued and {} solution",
                    generation,
                    pretty_print(total_generated),
                    pretty_print(total_abandoned),
                    pretty_print(to_search.len() as u64),
                    if best_solution.is_some() { "a" } else { "no" });
            }
        }

        if is_success(&prev.state)
        {
            match &best_solution
            {
                None =>
                {
                    if debug >= SearchDebugLevel::Solutions
                    {
                        println!("    Found first solution: {:?}", prev.state);
                    }

                    best_solution = Some(prev.state);
                },
                Some(cur_best) =>
                {
                    if compare_successes(&prev.state, cur_best) == Ordering::Less
                    {
                        if debug >= SearchDebugLevel::Solutions
                        {
                            println!("   Found better solution: {:?}", prev.state);
                        }

                        best_solution = Some(prev.state);
                    }
                    else
                    {
                        rejected_solutions += 1;
                    }
                },
            }
        }
        else
        {
            let should_generate = match &best_solution
            {
                None => true,
                Some(best_sol) => can_state_reach_better_than_solution(&prev.state, best_sol),
            };

            if should_generate
            {
                to_search.extend(neighbours(&prev.state)
                    .into_iter()
                    .map(|s|
                    {
                        total_generated += 1;
                        StateCompare { cmp: prev.cmp, generation: prev.generation + 1, state: s}
                    }));
            }
            else
            {
                total_abandoned += 1;
            }
        }
    }

    if debug >= SearchDebugLevel::FinalStats
    {
        println!("Result: {:?}", best_solution);
        println!("   Total states generated: {}", pretty_print(total_generated));
        println!("   Rejected solutions: {}", pretty_print(rejected_solutions));
        println!("   Abandoned states: {}", pretty_print(total_abandoned));
    }

    best_solution
}

struct StateCompare<'a, S, FC>
    where FC: Fn(&S, &S) -> Ordering
{
    cmp: &'a FC,
    generation: u64,
    state: S,
}

impl<'a, S, FC> PartialEq for StateCompare<'a, S, FC>
    where FC: Fn(&S, &S) -> Ordering
{
    fn eq(&self, other: &Self) -> bool
    {
        (self.cmp)(&self.state, &other.state) == Ordering::Equal
    }
}

impl <'a, S, FC> Eq for StateCompare<'a, S, FC>
    where FC: Fn(&S, &S) -> Ordering
{
}

impl<'a, S, FC> PartialOrd for StateCompare<'a, S, FC>
    where FC: Fn(&S, &S) -> Ordering
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering>
    {
        Some(self.cmp(other))
    }
}

impl<'a, S, FC> Ord for StateCompare<'a, S, FC>
    where FC: Fn(&S, &S) -> Ordering
{
    fn cmp(&self, other: &Self) -> Ordering
    {
        (self.cmp)(&self.state, &other.state)
    }
}

fn pretty_print(stat: u64) -> String
{
    let mut stat = stat;
    let mut parts = Vec::new();

    while stat > 1000
    {
        parts.insert(0, format!("{:03}", stat % 1000));
        stat = stat / 1000;
    }
    parts.insert(0, stat.to_string());

    parts.join(",")
}