const INPUT: &str = include_str!("input_1.txt");

fn fuels(input: &str) -> Vec<usize>
{
    input.split("\n")
        .filter(|s| !s.is_empty())
        .map(|s| s.parse::<usize>().unwrap())
        .collect()
}

fn fuel_required(mass: usize) -> usize
{
    let divided = mass / 3;

    if divided >= 2
    {
        return divided - 2;
    }
    else
    {
        return 0;
    }
}

fn part2_fuel_required(mass: usize) -> usize
{
    let mut extra = fuel_required(mass);
    let mut total = extra;

    loop
    {
        extra = fuel_required(extra);

        if extra == 0
        {
            return total;
        }

        total += extra;
    }
}

fn module_fuel_required(input: &str) -> usize
{
    fuels(input).iter().map(|a| fuel_required(*a)).fold(0, |a, b| a + b)
}

fn total_fuel_required(input: &str) -> usize
{
    fuels(input).iter().map(|a| part2_fuel_required(*a)).fold(0, |a, b| a + b)
}

fn main()
{
    assert_eq!(module_fuel_required("12\n"), 2);
    assert_eq!(module_fuel_required("14\n"), 2);
    assert_eq!(module_fuel_required("1969\n"), 654);
    assert_eq!(module_fuel_required("100756\n"), 33583);

    println!("Answer #1={}", module_fuel_required(INPUT));

    assert_eq!(total_fuel_required("14\n"), 2);
    assert_eq!(total_fuel_required("1969\n"), 966);
    assert_eq!(total_fuel_required("100756\n"), 50346);

    println!("Answer #2={}", total_fuel_required(INPUT));
}