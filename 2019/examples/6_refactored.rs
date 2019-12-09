use std::collections::HashMap;

const INPUT: &str = include_str!("input_6.txt");

struct Orbits
{
    orbits: HashMap<String, String>,
}

impl Orbits
{
    fn new(input: &str) -> Self
    {
        let lines = input
            .split("\n")
            .filter(|a| !a.is_empty())
            .collect::<Vec<_>>();

        let mut orbits = HashMap::new();

        for line in lines
        {
            let parts = line.split(")").collect::<Vec<_>>();

            orbits.insert(parts[1].to_string(), parts[0].to_string());
        }

        Orbits{ orbits }
    }

    fn orbits_from_to(&self, from: &String, to: &String) -> Option<usize>
    {
        let mut cur = to;
        let mut result = 0;
        loop
        {
            if let Some(parent) = self.orbits.get(cur)
            {
                cur = parent;
                result += 1;
                if cur == from
                {
                    return Some(result);
                }
            }
            else
            {
                // Cur has no parent
                return None
            }
        }
    }

    fn total_orbits(&self) -> usize
    {
        let com: String = "COM".to_string();
        let mut sum = 0;

        for (to, _from) in self.orbits.iter()
        {
            if let Some(count) = self.orbits_from_to(&com, to)
            {
                sum += count;
            }
        }

        sum
    }

    fn trans_between(&self, from: &String, to: &String) -> usize
    {
        let mut cur = to;
        loop
        {
            if let Some(a) = self.orbits_from_to(cur, to)
            {
                if let Some(b) = self.orbits_from_to(cur, from)
                {
                    return a + b - 2;
                }
            }
            
            if let Some(parent) = self.orbits.get(cur)
            {
                cur = parent;
            }
            else
            {
                assert!(false);
                unreachable!();
            }
        }
    }
}

fn part_1(input: &str) -> usize
{
    let orbits = Orbits::new(input);
    orbits.total_orbits()
}

fn part_2(input: &str) -> usize
{
    let orbits = Orbits::new(input);
    orbits.trans_between(&"YOU".to_owned(), &"SAN".to_owned())
}

fn main()
{
    const EXAMPLE_1: &str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\n";
    const EXAMPLE_2: &str = "COM)B\nB)C\nC)D\nD)E\nE)F\nB)G\nG)H\nD)I\nE)J\nJ)K\nK)L\nK)YOU\nI)SAN\n";

    assert_eq!(part_1(EXAMPLE_1), 42);

    let answer_1 = part_1(INPUT);
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 150150);

    assert_eq!(part_2(EXAMPLE_2), 4);

    let answer_2 = part_2(INPUT);
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 352);
}