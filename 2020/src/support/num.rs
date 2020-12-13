pub fn gcd(a: u64, b: u64) -> u64
{
    if b == 0
    {
        a
    }
    else
    {
        gcd(b, a % b)
    }
}

pub fn lcm(a: u64, b: u64) -> u64
{
    a * b / gcd(a, b)
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
        assert_eq!(gcd(48, 180), 12);
        assert_eq!(gcd(54, 24), 6);
    }

    #[test]
    fn test_lcm()
    {
        assert_eq!(lcm(2, 3), 6);
        assert_eq!(lcm(7, 13), 91);
        assert_eq!(lcm(12, 24), 24);
    }
}