fn phase(input: &Vec<u8>) -> Vec<u8>
{
    // Initially the pattern is [1, 0, -1, 0, 1, 0, -1...]
    // Then it becomes          [0, 1,  1, 0, 0, -1, -1 ...]
    //                          [0, 0,  1, 1, 1, 0, 0, 0, -1...]
    //
    // So the first range is special - for each output character, it
    // can be calculated by adding the next two values and removing the first value.
    // For each step we have to calculcate the rest of the values in full - but
    // these will get smaller with each step

    let mut output = input.clone();
    let len = input.len();

    let mut r1_start = 0;
    let mut r1_end = 0;
    let mut r1_sum = input[0] as i64;

    for index in 0..len
    {
        let span_len = index + 1;

        // Calculate the R1 sum (but not for the first index...)
        if index != 0
        {
            if (r1_end + 1) < len
            {
                r1_sum += input[r1_end + 1] as i64;
            }
            if (r1_end + 2) < len
            {
                r1_sum += input[r1_end + 2] as i64;
            }
            r1_sum -= input[r1_start] as i64;
            r1_start += 1;
            r1_end += 2;
        }
        assert_eq!(r1_end - r1_start + 1, span_len);

        //println!("Index {}, [{}..{}]={}", index, r1_start, r1_end, r1_sum);

        let mut index_sum = r1_sum;
        let mut pos = r1_end + 1 + span_len;
        let mut sign = -1 as i64;

        while pos < len
        {
            let mut span_end = pos + span_len;
            if span_end > len
            {
                span_end = len;
            }
            //println!("   Span [{}..{}] * {}", pos, span_end, sign);
            for p in pos..span_end
            {
                index_sum += sign * (input[p] as i64);
            }

            pos += 2 * span_len;
            sign *= -1;
        }

        if index_sum < 0
        {
            index_sum *= -1;
        }

        output[index] = (index_sum % 10) as u8;
    }

    output
}

fn chars_to_vec(input: &'static str) -> Vec<u8>
{
    input.chars().filter(|&ch| ch != '\n').map(|ch| (ch as u8) - ('0' as u8)).collect()
}

fn vec_to_chars(vec: &Vec<u8>) -> String
{
    vec.iter().map(|&i| (i + ('0' as u8)) as char).collect()
}

fn part_1(input: &'static str) -> String
{
    let mut state = chars_to_vec(input);

    for _i in 0..100
    {
        //println!("Phase {}, len={}", i, state.len());
        state = phase(&state);
    }

    let first_eight = state.drain(..).take(8).collect::<Vec<u8>>();

    vec_to_chars(&first_eight)
}

fn part_2(input: &'static str) -> String
{
    // The final digits of the output only depend on the final digits of
    // the inputs.
    // For example, for 16 digits input
    // o[8] = sum(i[8]..i[15])
    // o[9] = sum(i[9]..i[15])
    // ...
    // o[15] = i[15]

    // The digit offlset is large - so we only need
    // to calculate the final digits

    let input = chars_to_vec(input);

    let total_len = input.len() * 10000;

    let offset = input.iter().take(7).map(|&i| (i + ('0' as u8)) as char).collect::<String>().parse::<usize>().unwrap();

    assert!(offset > (total_len / 2));

    let to_calc = total_len - offset;

    let mut full_input = Vec::new();
    for _i in 0..10000
    {
        full_input.append(&mut input.clone());
    }

    let mut end_input = full_input.drain(..).skip(offset).collect::<Vec<u8>>();

    assert_eq!(to_calc, end_input.len());

    for _phase in 0..100
    {
        let mut sum: i64 = 0;

        for i in 0..to_calc
        {
            let index = to_calc - 1 - i;
            sum += end_input[index] as i64;

            end_input[index] = (sum.abs() % 10) as u8;
        }
    }

    vec_to_chars(&end_input.drain(..).take(8).collect())
}

fn main()
{
    const INPUT: &str = include_str!("input_16.txt");

    assert_eq!(phase(&vec![1, 2, 3, 4, 5, 6, 7, 8]), vec![4, 8, 2, 2, 6, 1, 5, 8]);
    assert_eq!(part_1("80871224585914546619083218645595"), "24176176".to_string());
    assert_eq!(part_1("19617804207202209144916044189917"), "73745418".to_string());
    assert_eq!(part_1("69317163492948606335995924319873"), "52432133".to_string());

    let answer_1 = part_1(INPUT);
    println!("Answer #1={}", answer_1);
    assert_eq!(answer_1, "27831665".to_string());

    assert_eq!(part_2("03036732577212944063491565474664"), "84462026".to_string());
    assert_eq!(part_2("02935109699940807407585447034323"), "78725270".to_string());
    assert_eq!(part_2("03081770884921959731165446850517"), "53553731".to_string());

    let answer_2 = part_2(INPUT);
    println!("Answer #2={}", answer_2);
    assert_eq!(answer_2, "36265589".to_string());
}