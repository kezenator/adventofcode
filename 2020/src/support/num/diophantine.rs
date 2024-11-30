use crate::support::alg::{RangeInc, RangeSet};

#[derive(Debug)]
pub struct Quadratic
{
    a: i64,
    b: i64,
    c: i64,
}

impl Quadratic
{
    pub fn new(a: i64, b: i64, c: i64) -> Self
    {
        Quadratic { a, b, c }
    }

    #[allow(unused)]
    pub fn eval(&self, x: i64) -> i64
    {
        self.a * x * x + self.b * x + self.c
    }

    pub fn solve(&self) -> DiophantineSolution
    {
        solve_single_quadratic(self.a, self.b, self.c)
    }
}

#[derive(Debug)]
pub struct DiophantineSolution
{
    pub zeros: RangeSet<i64>,
    pub positive: RangeSet<i64>,
    pub negative: RangeSet<i64>,
}

impl DiophantineSolution
{
    fn empty() -> Self
    {
        DiophantineSolution { zeros: RangeSet::new(), positive: RangeSet::new(), negative: RangeSet::new() }
    }

    fn insert_range(&mut self, range: RangeInc<i64>, signum: i64)
    {
        if signum < 0
        {
            self.negative.insert_range(range);
        }
        else if signum == 0
        {
            self.zeros.insert_range(range);
        }
        else
        {
            self.positive.insert_range(range);
        }
    }
}

pub fn solve_single_quadratic(a: i64, b: i64, c: i64) -> DiophantineSolution
{

    if (a == 0) && (b == 0)
    {
        // Constant: y = c
        let mut result = DiophantineSolution::empty();
        result.insert_range(RangeInc::all(), c.signum());
        result
    }
    else if a == 0
    {
        // Linear: y = bx + c
        let mut result = DiophantineSolution::empty();

        if (c % b) == 0
        {
            // Has an integer zero
            let zero = -c / b;

            result.insert_range(RangeInc::new_range(zero + 1, i64::MAX), b.signum());
            result.insert_range(RangeInc::new_range(zero, zero), 0);
            result.insert_range(RangeInc::new_range(i64::MIN, zero - 1), -b.signum());
        }
        else
        {
            // No integer zero
            let first_with_b_signum = -c / b;

            result.insert_range(RangeInc::new_range(first_with_b_signum, i64::MAX), b.signum());
            result.insert_range(RangeInc::new_range(i64::MIN, first_with_b_signum - 1), -b.signum());
        }

        result
    }
    else
    {
        // Quadratic: y = ax^2 + bx + c
        // Solve 0 = a * x^2 + b * x + c
        // => x = (1 / 2a) * (-b +/- sqrt(b^2 - 4ac))

        let mut result = DiophantineSolution::empty();

        let discriminant = (b * b) - (4 * a * c);

        if discriminant > 0
        {
            // Two roots at -b/2a +/- sqrt(dis)/2a

            // Ensure positive - we swap again by saving the original a_signum
            let a_signum = a.signum();
            let (mut a, mut b, mut c) = (a, b, c);
            if a_signum < 0
            {
                (a, b, c) = (-a, -b, -c);
            }
            let (a, b, c) = (a, b, c);

            // Find an extreme min and max that are guaranteed
            // to be ouside the roots
            let mut min = ((-b - isqrt(discriminant) - 1) / (2 * a)) - 1;
            let mut max = ((-b + isqrt(discriminant) + 1) / (2 * a)) + 1;
            while (min < max) && (eval_quad(a, b, c, min) > 0)
            {
                min += 1;
            }
            while (min < max) && (eval_quad(a, b, c, max) > 0)
            {
                max -= 1;
            }

            if min >= max
            {
                // There are actually zero integer roots
                result.insert_range(RangeInc::all(), a_signum);
            }
            else
            {
                // -inf .. (min - 1) is def "positive" (* a_signum)
                // (max + 1) is def "positive" (* a_signum)

                result.insert_range(RangeInc::new_range(i64::MIN, min - 1), a_signum);
                result.insert_range(RangeInc::new_range(max + 1, i64::MAX), a_signum);

                // Check if min/max are zeros

                if eval_quad(a, b, c, min) == 0
                {
                    result.insert_range(RangeInc::new_range(min, min), 0);
                    min += 1;
                }

                if eval_quad(a, b, c, max) == 0
                {
                    result.insert_range(RangeInc::new_range(max, max), 0);
                    max -= 1;
                }

                // Now - any remaining range is "negative" (* a_signum)

                if min <= max
                {
                    result.insert_range(RangeInc::new_range(min, max), -a_signum);
                }
            }
        }
        else if discriminant == 0
        {
            // One root

            if (b % (2 * a)) == 0
            {
                // One integer root
                let root = -b / (2 * a);
                result.insert_range(RangeInc::new_range(i64::MIN, root - 1), a.signum());
                result.insert_range(RangeInc::new_range(root, root), 0);
                result.insert_range(RangeInc::new_range(root + 1, i64::MAX), a.signum());
            }
            else
            {
                // One non-integer root
                result.insert_range(RangeInc::all(), a.signum());
            }
        }
        else
        {
            // Zero roots
            result.insert_range(RangeInc::all(), a.signum());
        }

        result
    }
}

fn eval_quad(a: i64, b: i64, c: i64, x: i64) -> i64
{
    a * x * x + b * x + c
}

fn isqrt(val: i64) -> i64
{
    // https://users.rust-lang.org/t/integer-square-root/96/7
    // Apparently: valid for x << (1 << 52)

    assert!(val >= 0);
    assert!(val <= (1 << 52));
    (val as f64).sqrt() as i64
}

#[cfg(test)]
mod tests
{
    use super::*;

    fn confirm_disjoint(a: &RangeSet<i64>, b: &RangeSet<i64>)
    {
        let mut remaining = a.clone();
        for range in b.ranges()
        {
            remaining.remove_range(range);
        }
        assert!(*a == remaining);
    }

    fn confirm_ranges(solution: &DiophantineSolution)
    {
        let mut all = RangeSet::new();
        all.insert_set(&solution.zeros);
        all.insert_set(&solution.positive);
        all.insert_set(&solution.negative);
        assert!(all == RangeSet::all());

        confirm_disjoint(&solution.positive, &solution.negative);
        confirm_disjoint(&solution.positive, &solution.zeros);
        confirm_disjoint(&solution.zeros, &solution.negative);
    }

    fn confirm_solution(a: i64, b: i64, c: i64, results: Vec<(i64, i64)>, num_zeros: usize)
    {
        let solution = &solve_single_quadratic(a, b, c);

        confirm_ranges(solution);

        for (x, signum) in results
        {
            if signum > 0
            {
                assert!(!solution.negative.contains_value(x));
                assert!(!solution.zeros.contains_value(x));
                assert!(solution.positive.contains_value(x));
            }
            else if signum < 0
            {
                assert!(solution.negative.contains_value(x));
                assert!(!solution.zeros.contains_value(x));
                assert!(!solution.positive.contains_value(x));
            }
            else
            {
                assert!(!solution.negative.contains_value(x));
                assert!(solution.zeros.contains_value(x));
                assert!(!solution.positive.contains_value(x));
            }
        }

        if num_zeros > 2
        {
            assert!(solution.zeros == RangeSet::all());
        }
        else
        {
            assert!(solution.zeros != RangeSet::all());
            assert!(solution.zeros.ranges().count() <= 2);
            assert!(solution.zeros.values().count() == num_zeros);
        }
    }

    #[test]
    fn test_diophantine_constant()
    {
        // Constant

        confirm_solution(0, 0, -1, vec![(-1, -1), (0, -1), (1, -1)], 0);
        confirm_solution(0, 0, 0, vec![(-1, 0), (0, 0), (1, 0)], 3);
        confirm_solution(0, 0, 1, vec![(-1, 1), (0, 1), (1, 1)], 0);

        // Linear

        confirm_solution(0, 1, 0, vec![(-1, -1), (0, 0), (1, 1)], 1);
        confirm_solution(0, 1, 1, vec![(-2, -1), (-1, 0), (0, 1)], 1);
        confirm_solution(0, -1, 0, vec![(-1, 1), (0, 0), (1, -1)], 1);
        confirm_solution(0, -1, 1, vec![(0, 1), (1, 0), (2, -1)], 1);

        confirm_solution(0, 3, 6, vec![(-3, -1), (-2, 0), (-1, 1), (0, 1)], 1);
        confirm_solution(0, 3, 7, vec![(-3, -1), (-2, 1), (-1, 1), (0, 1)], 0);

        // Quadratic - two roots

        confirm_solution(4, -6, 0, vec![(-2, 1), (-1, 1), (0, 0), (1, -1), (2, 1), (3, 1)], 1);
        confirm_solution(-1, 4, -3, vec![(-2, -1), (-1, -1), (0, -1), (1, 0), (2, 1), (3, 0), (4, -1)], 2);

        // Quadratic - one root

        confirm_solution(1, 0, 0, vec![(-1, 1), (0, 0), (1, 1)], 1);
        confirm_solution(4, -4, 1, vec![(-1, 1), (0, 1), (1, 1), (2, 1), (3, 1)], 0);

        // Quadratic - zero roots

        confirm_solution(1, 0, 1, vec![(-1, 1), (0, 1), (1, 1)], 0);
        confirm_solution(-1, 0, -1, vec![(-1, -1), (0, -1), (1, -1)], 0);
    }
}