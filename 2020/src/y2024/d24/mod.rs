
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");

#[derive(Hash, PartialEq, Eq, Clone, Debug)]
enum Op
{
    AND,
    OR,
    XOR,
}

impl ToString for Op
{
    fn to_string(&self) -> String
    {
        match self
        {
            Op::AND => "AND",
            Op::OR => "OR",
            Op::XOR => "XOR",
        }.to_owned()
    }
}

#[derive(Clone, Debug)]
enum Wire
{
    Input(usize),
    Gate(String, Op,  String),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
enum EquationTerm
{
    Input(String),
    Gate(Op, Vec<Equation>),
}

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Equation
{
    term: EquationTerm,
    depth: usize,
    string: String,
}

fn flatten_equations(result: &mut Vec<Equation>, op: &Op, eq: Equation)
{
    match eq.term
    {
        EquationTerm::Gate(eq_op, eq_terms) 
            if eq_op == *op =>
        {
            for term in eq_terms
            {
                flatten_equations(result, op, term);
            }
        },
        _ => result.push(eq),
    }
}

impl Equation
{
    fn new_input(input: String) -> Self
    {
        let term = EquationTerm::Input(input.clone());
        let depth = 0;
        let string = input;
        Equation { term, depth, string }
    }

    fn new_input_n(prefix: &str, num: usize) -> Self
    {
        Equation::new_input(format!("{}{:02}", prefix, num))
    }

    fn new_gate(left: Equation, op: Op, right: Equation) -> Self
    {
        let depth = left.depth.max(right.depth) + 1;

        let mut sub_eqs = Vec::new();
        flatten_equations(&mut sub_eqs, &op, left);
        flatten_equations(&mut sub_eqs, &op, right);
        sub_eqs.sort_by(|a, b| a.string.cmp(&b.string));

        let string = format!("{}{}({})",
            op.to_string(),
            sub_eqs.len(),
            sub_eqs.iter().map(|e| e.string.clone()).join(","));

        let term = EquationTerm::Gate(op, sub_eqs);

        Equation { depth, term, string }
    }

    fn new_gate_from_vec(op: Op, eqs: Vec<Equation>) -> Self
    {
        let depth = eqs.iter()
            .map(|e| e.depth)
            .max().unwrap() + 1;
        let string = format!("{}{}({})",
            op.to_string(),
            eqs.len(),
            eqs.iter().map(|e| e.string.clone()).join(","));
        let term = EquationTerm::Gate(op, eqs);

        Equation { depth, term, string }
    }
}

impl ToString for Equation
{
    fn to_string(&self) -> String
    {
        self.string.clone()
    }
}

struct Wires
{
    wires: HashMap<String, Wire>,
}

impl Wires
{
    fn new(input: &str) -> Self
    {
        let groups = input_to_groups(input);

        let mut wires = HashMap::new();

        for (name, fixed) in groups[0].iter()
            .map(|l| scan(l).until(": ").parse().remaining().parse())
        {
            wires.insert(name, Wire::Input(fixed));
        }

        for (left, op, right, name) in groups[1].iter()
            .map(|l|
            {
                scan(l)
                    .until_whitespace().parse()
                    .until_whitespace().parse::<String>()
                    .until(" -> ").parse()
                    .remaining().parse()
            })
        {
            let op = match op.as_str()
            {
                "AND" => Op::AND,
                "OR" => Op::OR,
                "XOR" => Op::XOR,
                _ => unreachable!(),
            };
            wires.insert(name, Wire::Gate(left, op, right));
        }

        Wires { wires }
    }

    fn calculate_all(& self) -> HashMap<String, usize>
    {
        let func = |input: &String, memorized: &Memorized<String, usize>| -> usize
        {
            match self.wires.get(input).unwrap()
            {
                Wire::Input(f) => *f,
                Wire::Gate(left, op, right) =>
                {
                    let left = memorized.get(left);
                    let right = memorized.get(right);
                    match op
                    {
                        Op::AND => left & right,
                        Op::OR => left | right,
                        Op::XOR => left ^ right,
                    }
                }
            }
        };
        let memorized = Memorized::new(&func);

        self.wires.keys()
            .map(|n| (n.clone(), memorized.get(n)))
            .collect()
    }

    fn output_value(&self) -> usize
    {
        let values = self.calculate_all();
        let output_names = values.keys()
            .filter(|k| k.starts_with('z'))
            .collect_vec();
        let num_outputs = output_names.len();
        let mut binary_string = String::new();
        for i in (0..num_outputs).rev()
        {
            let output_name = format!("z{:02}", i);
            binary_string.push_str(&values.get(&output_name).unwrap().to_string());
        }
        usize::from_str_radix(&binary_string, 2).unwrap()
    }

    fn rewritten_equation(&self, name: &String, rewrites: &HashMap<String, String>) -> Equation
    {
        let rewritten_name = match rewrites.get(name)
        {
            None => name,
            Some(rewritten) => rewritten,
        };

        match self.wires.get(rewritten_name).unwrap()
        {
            Wire::Input(_) => Equation::new_input(name.clone()),
            Wire::Gate(left, op, right) =>
            {
                Equation::new_gate(
                    self.rewritten_equation(left, rewrites),
                    op.clone(),
                    self.rewritten_equation(right, rewrites))
            }
        }
    }

    fn create_carry_eq(&self, output: usize) -> Equation
    {
        if output == 0
        {
            Equation::new_gate(
                Equation::new_input_n("x", 0),
                Op::AND,
                Equation::new_input_n("y", 0))
        }
        else
        {
            // (carry-1 && x^y) || x&y
            Equation::new_gate(
                Equation::new_gate(
                    self.create_carry_eq(output - 1),
                    Op::AND,
                    Equation::new_gate(
                        Equation::new_input_n("x", output),
                        Op::XOR,
                        Equation::new_input_n("y", output))),
                Op::OR,
                Equation::new_gate(
                    Equation::new_input_n("x", output),
                    Op::AND,
                    Equation::new_input_n("y", output)))
        }
    }

    fn create_output_eq(&self, output: usize) -> Equation
    {
        if output == 0
        {
            Equation::new_gate(
                Equation::new_input_n("x", 0),
                Op::XOR,
                Equation::new_input_n("y", 0))
        }
        else
        {
            Equation::new_gate(
                self.create_carry_eq(output - 1),
                Op::XOR,
                Equation::new_gate(
                    Equation::new_input_n("x", output),
                    Op::XOR,
                    Equation::new_input_n("y", output)))
        }
    }

    fn find_errors(&self) -> HashSet<String>
    {
        let mut rewrites = HashMap::new();

        let num_outputs = self.wires.keys()
            .filter(|w| w.starts_with("z"))
            .count();

        // Keep finding new errors until we can't find
        // any more
        //let mut loop_counter = 0;
        loop
        {
            //loop_counter += 1;

            // Create mappings from each name to their equation,
            // and each equation back to their current name

            let mut actual_n_to_eq = HashMap::new();
            let mut actual_eq_to_n = HashMap::new();

            for name in self.wires.keys()
            {
                let eq = self.rewritten_equation(name, &rewrites);
                actual_eq_to_n.insert(eq.to_string(), name.clone());
                actual_n_to_eq.insert(name.clone(), eq);
            }
            
            // Recursively go through each output and it's sub-equations
            // and see if they match the expected equation.
            // Collect a list - including the depths where the error occured -
            // so we can find the error closest to the inputs to fix

            let mut errors = HashSet::<(usize, String, String, usize)>::new();

            for i in 0..num_outputs
            {
                // There's only carry for the final output
                let expected_eq = if i == (num_outputs - 1)
                {
                    self.create_carry_eq(i - 1)
                }
                else
                {
                    self.create_output_eq(i)
                };

                let check_eq = |input: &(Equation, String), memorized: &Memorized<(Equation, String), HashSet<(usize, String, String, usize)>>| -> HashSet<(usize, String, String, usize)>
                {
                    let mut result = HashSet::new();
                    let actual_eq = actual_n_to_eq.get(&input.1).unwrap();
                    let actual_eq_str = actual_eq.to_string();
                    let expected_eq = input.0.to_string();
                    if actual_eq_str != expected_eq
                    {
                        //println!("\n\n\n\nOutput: {}\nExpected:\n{}\n\nName:\n{}\n\nActual:\n{}", i, expected_eq, input.1, actual_eq_str);
                        if let Some(correct_name) = actual_eq_to_n.get(&expected_eq)
                        {
                            // There's a correct other equation we already
                            // know about to use
                            result.insert((input.0.depth, input.1.clone(), correct_name.clone(), i));
                        }
                        else if let (EquationTerm::Gate(eop, eterms),
                                        EquationTerm::Gate(aop, aterms))
                            = (&input.0.term, &actual_eq.term)
                        {
                            if *eop == *aop
                            {
                                // We need to match the sub-terms.
                                // First - map the known equations from their string form
                                let mut eterms_by_str = eterms.iter()
                                    .map(|e| (e.string.clone(), e.clone()))
                                    .collect::<HashMap<_, _>>();
                                let mut aterms_by_str = aterms.iter()
                                    .map(|e| (e.string.clone(), e.clone()))
                                    .collect::<HashMap<_, _>>();
                                // Now remove the matching items
                                for a in aterms.iter()
                                {
                                    if eterms_by_str.contains_key(&a.string)
                                        && aterms_by_str.contains_key(&a.string)
                                    {
                                        eterms_by_str.remove(&a.string);
                                        aterms_by_str.remove(&a.string);
                                    }
                                }
                                //println!("OP: {:?}", eop);
                                //println!("ETERMS: {:?}", eterms_by_str.keys().collect_vec());
                                //println!("ATERMS: {:?}", aterms_by_str.keys().collect_vec());
                                // Hopefully there's only one left....
                                assert!(aterms_by_str.len() == 1);
                                // Hopefully this has a wire name....
                                let awire_name = actual_eq_to_n.get(aterms_by_str.keys().next().unwrap());
                                assert!(awire_name.is_some());
                                // There must be at least one eterm - for my input
                                // some errors had more than one eterm left and I had
                                // to combine the back by eop (above)
                                assert!(eterms_by_str.len() >= 1);

                                let sub_expected = if eterms_by_str.len() == 1
                                {
                                    eterms_by_str.values().next().unwrap().clone()
                                }
                                else
                                {
                                    Equation::new_gate_from_vec(
                                        eop.clone(),
                                        eterms_by_str.values().cloned().collect_vec())
                                };
                                //println!("SUB-E: {}", sub_expected.string);
                                
                                // Recursive descent into this equation
                                for r in memorized.get(&(
                                    sub_expected,
                                    awire_name.unwrap().clone()))
                                {
                                    result.insert(r);
                                }
                            }
                            else
                            {
                                // Oh dear....
                                unreachable!();
                            }
                        }
                        else
                        {
                            // Oh dear....
                            unreachable!();
                        }
                    }
                    result
                };
                let check_eq_m = Memorized::new(&check_eq);

                let mut any_from_this_output = false;
                for e in check_eq_m.get(&(expected_eq, format!("z{:02}", i)))
                {
                    errors.insert(e);
                    any_from_this_output = true;
                }

                if any_from_this_output
                {
                    // Always fix errors in the simpler lower-bit
                    // outputs first
                    break;
                }
            }

            //println!("Loop {}", loop_counter);
            //println!("Rewrites: {:?}", rewrites);
            //println!("New Errors: {:?}", errors);
            if errors.is_empty()
            {
                break;
            }

            let best_info = errors.iter()
                .map(|(depth, _, _, output)| (num_outputs - *output, *depth))
                .max().unwrap();

            let best_error = errors.into_iter()
                .filter(|(depth, _, _, output)| (num_outputs - *output, *depth) == best_info)
                .next().unwrap();

            //println!("Best: {:?}", best_error);
            //println!("A: {}", self.equation(&best_error.1).to_string());
            //println!("B: {}", self.equation(&best_error.2).to_string());

            let existing1 = rewrites.insert(best_error.1.clone(), best_error.2.clone());
            let existing2 = rewrites.insert(best_error.2, best_error.1);
            assert!(existing1.is_none());
            assert!(existing2.is_none());
        }

        // Now - return all of the re-written items

        assert!(rewrites.len() == 8);
        rewrites.keys().cloned().collect()
    }
}

fn part_1(input: &str) -> usize
{
    let wires = Wires::new(input);
    wires.output_value()
}

fn part_2(input: &str) -> String
{
    let wires = Wires::new(input);
    wires.find_errors().into_iter().sorted().join(",")
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(24)
        .example(|| Answer {
            calculated: part_1(EXAMPLE),
            expected: 2024,
        })
        .part_1(|input| Answer {
            calculated: part_1(input),
            expected: 55114892239566usize,
        })
        .part_2(|input| Answer {
            calculated: part_2(input),
            expected: "cdj,dhm,gfm,mrb,qjd,z08,z16,z32",
        })
}
