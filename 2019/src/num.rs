pub fn gcd(a: i64, b: i64) -> i64
{
    assert!(a >= 0);
    assert!(b >= 0);

    let mut state = (a, b);

    loop
    {
        if state.1 == 0
        {
            return state.0
        }
        else
        {
            state = (state.1, state.0 % state.1);
        }
    }
}

pub fn lcm(a: i64, b: i64) -> i64
{
    assert!(a != 0 || b != 0);

    (a / gcd(a, b)) * b
}

#[cfg(test)]
mod tests
{
    use super::*;

    #[test]
    fn test_gcd()
    {
        assert_eq!(gcd(0, 0), 0);
        assert_eq!(gcd(1, 1), 1);
        assert_eq!(gcd(2, 2), 2);
        assert_eq!(gcd(2, 3), 1);

        assert_eq!(gcd(2 * 3 * 5 * 7, 3 * 5 * 7 * 11), 3 * 5 * 7);
        assert_eq!(gcd(97, 101), 1);
        assert_eq!(gcd(16, 32), 16);
        assert_eq!(gcd(16 * 7 * 11 * 13, 32 * 11 * 13 * 17), 16 * 11 * 13);

        assert_eq!(lcm(1, 1), 1);
        assert_eq!(lcm(1, 2), 2);
        assert_eq!(lcm(2, 2), 2);
        assert_eq!(lcm(2, 3), 6);
        assert_eq!(lcm(2 * 2 * 3 * 5 * 7, 2 * 3 * 7 * 11), 2 * 2* 3 * 5 * 7 * 11);
    }
}