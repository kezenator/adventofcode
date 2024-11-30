use crate::support::*;

const EXAMPLE: &str = include_str!("example.txt");
const EXAMPLE_REASSEMBLED: &str = include_str!("example_reassembled.txt");

#[derive(Clone)]
struct Tile
{
    num: u64,
    grid: CharGrid,
}

impl Tile
{
    fn new(input: &Vec<String>) -> Self
    {
        let (num,_) = scan(&input[0]).skip(5).take_digits().parse::<u64>().remaining().parse::<String>();

        let grid = CharGrid::new_from_input(&input[1..].join("\n"), '.');

        Tile { num, grid }
    }
}

fn borders_x4(grid: &CharGrid) -> Vec<String>
{
    let mut a = String::new();
    let mut b = String::new();
    let mut c = String::new();
    let mut d = String::new();

    for i in 0..10
    {
        a.push(grid.get_char(&Point::new(i, 0)));
        b.push(grid.get_char(&Point::new(9, i)));
        c.push(grid.get_char(&Point::new(i, 9)));
        d.push(grid.get_char(&Point::new(0, i)));
    }

    vec![a, b, c, d]
}

fn borders_x8(grid: &CharGrid) -> Vec<String>
{
    let x4 = borders_x4(grid);

    let a = x4[0].clone();
    let b = x4[1].clone();
    let c = x4[2].clone();
    let d = x4[3].clone();

    let ar = a.chars().rev().collect();
    let br = b.chars().rev().collect();
    let cr = c.chars().rev().collect();
    let dr = d.chars().rev().collect();

    vec![a, b, c, d, ar, br, cr, dr]
}

fn grid_variants(grid: &CharGrid) -> Vec<CharGrid>
{
    let mut result = Vec::new();

    let mut g = grid.clone();

    for _ in 0..4
    {
        result.push(g.clone());
        result.push(g.flip_horizontally());
        g = g.rotate_cw_90();
    }

    result
}

fn corner_tiles(input: &str) -> Vec<Tile>
{
    let tiles = input_to_groups(input)
        .iter()
        .map(|v| Tile::new(v))
        .collect::<Vec<_>>();

    let mut corners = Vec::new();

    for t1 in tiles.iter()
    {
        let borders = borders_x8(&t1.grid);

        let mut matches = 0;

        for t2 in tiles.iter()
        {
            if t1.num != t2.num
            {
                for t2b in borders_x4(&t2.grid).into_iter()
                {
                    if borders.iter().filter(|&t1b| *t1b == *t2b).count() != 0
                    {
                        matches += 1;
                    }
                }
            }
        }

        if matches == 2
        {
            corners.push(t1.clone());
        }
    }

    assert_eq!(corners.len(), 4);

    corners
}

fn match_index(tiles: &Vec<Tile>, cur: &(u64, CharGrid), border_x4_index: usize) -> Option<(u64, CharGrid)>
{
    let to_match = borders_x4(&cur.1)[border_x4_index].clone();

    let rev_index = (border_x4_index + 2) % 4;

    for t in tiles
    {
        if t.num != cur.0
            && borders_x8(&t.grid).contains(&to_match)
        {
            //println!("Match: tile {}/index {}/border {}", cur.0, border_x4_index, to_match);
            //println!("Found: {}", t.num);
            //println!("Need x4 index: {}", rev_index);
            //println!("x8: {:?}", borders_x8(&t.grid));

            for g in grid_variants(&t.grid)
            {
                //println!();
                //println!("x4: {:?}", borders_x4(&g));

                if borders_x4(&g)[rev_index] == to_match
                {
                    return Some((t.num, g.clone()));
                }
            }
            unreachable!();
        }
    }

    None
}

fn match_right(tiles: &Vec<Tile>, cur: &(u64, CharGrid)) -> Option<(u64, CharGrid)>
{
    match_index(tiles, cur, 1)
}

fn match_below(tiles: &Vec<Tile>, cur: &(u64, CharGrid)) -> Option<(u64, CharGrid)>
{
    match_index(tiles, cur, 2)
}

fn build_full_image(input: &str) -> CharGrid
{
    let all_tiles = input_to_groups(input)
        .iter()
        .map(|v| Tile::new(v))
        .collect::<Vec<_>>();

    let num_tiles_per_side: i64 = if all_tiles.len() == 9
    {
        3
    }
    else
    {
        12
    };
    assert_eq!(all_tiles.len() as i64, num_tiles_per_side * num_tiles_per_side);

    let first_corner = corner_tiles(input)
        .into_iter()
        .next()
        .unwrap();

    let first_corner_num = first_corner.num;
    let first_corner = grid_variants(&first_corner.grid)
        .into_iter()
        .filter(|t|
            {
                match_right(&all_tiles, &(first_corner_num, t.clone())).is_some()
                    && match_below(&all_tiles, &(first_corner_num, t.clone())).is_some()
            })
        .next()
        .unwrap();

    let mut result = CharGrid::new_from_fill(
        (8 * num_tiles_per_side) as usize,
        (8 * num_tiles_per_side) as usize,
        '.');

    let mut prev_tile = (first_corner_num, first_corner.clone());
    let mut prev_row_start = (first_corner_num, first_corner.clone());

    for y in 0..num_tiles_per_side
    {
        for x in 0..num_tiles_per_side
        {
            let insert = if y == 0 && x == 0
            {
                (first_corner_num, first_corner.clone())
            }
            else if x == 0
            {
                match_below(&all_tiles, &prev_row_start).unwrap()
            }
            else
            {
                match_right(&all_tiles, &prev_tile).unwrap()
            };

            //println!("Next tile: {}", insert.0);

            prev_tile = insert.clone();
            
            if x == 0
            {
                prev_row_start = insert.clone();
            }

            for iy in 0..8
            {
                for ix in 0..8
                {
                    result.put_char(
                        &Point::new(8 * x + ix, 8 * y + iy),
                        insert.1.get_char(&Point::new(ix + 1, iy + 1)));
                }
            }

            //println!("{}", result.to_string());
        }
    }

    result
}

fn check_reassembly(input: &str, expected: &str) -> bool
{
    let calculated = build_full_image(input);
    let expected = CharGrid::new_from_input(expected, '.').to_string();

    grid_variants(&calculated)
        .into_iter()
        .filter(|g| g.to_string() == expected)
        .count() != 0
}

fn mark_sea_monsters(grid: &mut CharGrid) -> bool
{
    let mut result = false;

    let matches = |ch: char| -> bool
    {
        ch == '#' || ch == 'O'
    };

    let points = vec![
        Point::new(0, 1),
        Point::new(1, 2),
        Point::new(4, 2),
        Point::new(5, 1),
        Point::new(6, 1),
        Point::new(7, 2),
        Point::new(10, 2),
        Point::new(11, 1),
        Point::new(12, 1),
        Point::new(13, 2),
        Point::new(16, 2),
        Point::new(17, 1),
        Point::new(18, 1),
        Point::new(18, 0),
        Point::new(19, 1),
    ];
    let req = points.len();

    for y in 0..(grid.get_height() - 1)
    {
        for x in 0..(grid.get_width() - 18)
        {
            if points
                .iter()
                .filter(|p| matches(grid.get_char(&Point::new(x + p.x, y + p.y))))
                .count() == req
            {
                // Found one - mark it

                result = true;

                for p in points.iter()
                {
                    grid.put_char(&Point::new(x + p.x, y + p.y), 'O');
                }
            }
        }
    }

    result
}

fn part_1(input: &str) -> u64
{
    corner_tiles(input)
        .into_iter()
        .map(|t| t.num)
        .product()
}

fn part_2(input: &str) -> usize
{
    for mut grid in grid_variants(&build_full_image(input))
    {
        if mark_sea_monsters(&mut grid)
        {
            //println!("{}", grid.to_string());

            return grid
                .all_chars()
                .into_iter()
                .filter(|&ch| ch == '#').count();
        }
    }

    unreachable!();
}

pub fn puzzles() -> PuzzleDay
{
    puzzle_day(20)
        .example(|| Answer { calculated: part_1(EXAMPLE), expected: 20899048083289u64, })
        .part_1(|input| Answer { calculated: part_1(input), expected: 15006909892229u64, })
        .example(|| Answer { calculated: check_reassembly(EXAMPLE, EXAMPLE_REASSEMBLED), expected: true, })
        .example(|| Answer { calculated: part_2(EXAMPLE), expected: 273, })
        .part_2(|input| Answer { calculated: part_2(input), expected: 2190, })
}
