use std::collections::HashMap;

#[derive(Clone)]
struct Equation
{
    target: String,
    quantity: i64,
    ingredients: Vec<(String, i64)>,
}

fn parse_pair(input: &'static str) -> (String, i64)
{
    let parts = input.trim().split(" ").collect::<Vec<_>>();
    (parts[1].to_string(), parts[0].parse::<i64>().unwrap())
}

impl Equation
{
    fn new(input: &'static str) -> Self
    {
        let s1 = input.split("=>").collect::<Vec<_>>();
        let s2 = s1[0].split(",").collect::<Vec<_>>();

        let target = parse_pair(s1[1]);
        let ingredients = s2.iter().map(|s| parse_pair(s)).collect::<Vec<_>>();

        Equation
        {
            target: target.0,
            quantity: target.1,
            ingredients,
        }
    }
}

struct Reactor
{
    equations: HashMap<String, Equation>,
    quantities: HashMap<String, i64>,
    total_made: HashMap<String, i64>,
}

impl Reactor
{
    fn new(input: &'static str) -> Self
    {
        let lines = input.split('\n').filter(|l| !l.is_empty()).collect::<Vec<_>>();
        let equations = lines.iter().map(|l| Equation::new(l)).map(|e| (e.target.clone(), e.clone())).collect::<HashMap<String, Equation>>();
        let mut quantities: HashMap<String, i64> = HashMap::new();
        let mut total_made: HashMap<String, i64> = HashMap::new();

        for (key, _) in equations.iter()
        {
            quantities.insert(key.clone(), 0);
            total_made.insert(key.clone(), 0);
        }

        quantities.insert("ORE".to_string(), 0);
        total_made.insert("ORE".to_string(), 0);

        Reactor { equations, quantities, total_made }
    }

    fn make(&mut self, chem: &String, num: i64)
    {
        //println!("Need {} {}", num, chem);

        let existing = *self.quantities.get(chem).unwrap();

        if existing < num
        {
            if chem == "ORE"
            {
                let to_make = num - existing;

                //println!("Making {} {}", to_make, chem);

                *self.quantities.get_mut(chem).unwrap() += to_make;
                *self.total_made.get_mut(chem).unwrap() += to_make;
            }
            else
            {
                let equation = self.equations.get(chem).unwrap().clone();
                let mut to_make = num - existing;
                if to_make % equation.quantity != 0
                {
                    to_make += equation.quantity - (to_make % equation.quantity);
                }

                let times = to_make / equation.quantity;

                //println!("Making {} {} ({} times equation)", to_make, chem, times);

                for (o_name, o_quant) in equation.ingredients
                {
                    self.make(&o_name, o_quant * times);
                }

                *self.quantities.get_mut(chem).unwrap() += to_make;
                *self.total_made.get_mut(chem).unwrap() += to_make;
            }
        }

        //println!("Using {} {}", num, chem);
        *self.quantities.get_mut(chem).unwrap() -= num;
    }
}

fn part_1(input: &'static str) -> i64
{
    let mut reactor = Reactor::new(input);
    reactor.make(&"FUEL".to_string(), 1);
    *reactor.total_made.get(&"ORE".to_string()).unwrap()
}

fn part_2(input: &'static str) -> i64
{
    const TARGET: i64 = 1000000000000;
    let ore = "ORE".to_string();
    let fuel = "FUEL".to_string();

    let mut reactor = Reactor::new(input);

    reactor.make(&fuel, 1);

    let required_for_one = *reactor.total_made.get(&ore).unwrap();

    loop
    {
        let cur_made = *reactor.total_made.get(&ore).unwrap();
        if cur_made >= TARGET
        {
            return *reactor.total_made.get(&fuel).unwrap() - 1;
        }

        let mut to_make = (TARGET - cur_made) / required_for_one / 3;
        if to_make < 1
        {
            to_make = 1;
        }

        reactor.make(&fuel, to_make);

        //println!("{} (Made {} this time)", *reactor.total_made.get(&ore).unwrap(), to_make);
    }
}

fn main()
{
    const INPUT: &str = include_str!("input_14.txt");

    const EXAMPLE_1: &str = "10 ORE => 10 A\n1 ORE => 1 B\n7 A, 1 B => 1 C\n7 A, 1 C => 1 D\n7 A, 1 D => 1 E\n7 A, 1 E => 1 FUEL\n";
    const EXAMPLE_2: &str = "9 ORE => 2 A\n8 ORE => 3 B\n7 ORE => 5 C\n3 A, 4 B => 1 AB\n5 B, 7 C => 1 BC\n4 C, 1 A => 1 CA\n2 AB, 3 BC, 4 CA => 1 FUEL\n";
    const EXAMPLE_3: &str = "157 ORE => 5 NZVS\n165 ORE => 6 DCFZ\n44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL\n12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ\n179 ORE => 7 PSHF\n177 ORE => 5 HKGWZ\n7 DCFZ, 7 PSHF => 2 XJWVT\n165 ORE => 2 GPVTF\n3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT\n";
    const EXAMPLE_4: &str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG\n17 NVRVD, 3 JNWZP => 8 VPVL\n53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL\n22 VJHF, 37 MNCFX => 5 FWMGM\n139 ORE => 4 NVRVD\n144 ORE => 7 JNWZP\n5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC\n5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV\n145 ORE => 6 MNCFX\n1 NVRVD => 8 CXFTF\n1 VJHF, 6 MNCFX => 4 RFSQX\n176 ORE => 6 VJHF\n";
    const EXAMPLE_5: &str = "171 ORE => 8 CNZTR\n7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL\n114 ORE => 4 BHXH\n14 VRPVC => 6 BMBT\n6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL\n6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT\n15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW\n13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW\n5 BMBT => 4 WPTQ\n189 ORE => 9 KTJDG\n1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP\n12 VRPVC, 27 CNZTR => 2 XDBXC\n15 KTJDG, 12 BHXH => 5 XCVML\n3 BHXH, 2 VRPVC => 7 MZWV\n121 ORE => 7 VRPVC\n7 XCVML => 6 RJRHP\n5 BHXH, 4 VRPVC => 5 LTCX\n";

    assert_eq!(part_1(EXAMPLE_1), 31);
    assert_eq!(part_1(EXAMPLE_2), 165);
    assert_eq!(part_1(EXAMPLE_3), 13312);
    assert_eq!(part_1(EXAMPLE_4), 180697);
    assert_eq!(part_1(EXAMPLE_5), 2210736);

    let answer_1 = part_1(INPUT);
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, 469536);

    assert_eq!(part_2(EXAMPLE_3), 82892753);
    assert_eq!(part_2(EXAMPLE_4), 5586022);
    assert_eq!(part_2(EXAMPLE_5), 460664);

    let answer_2 = part_2(INPUT);
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, 3343477);
}