use super::range_inc::*;
use itertools::*;
use std::fmt::Debug;
use std::iter::{IntoIterator, Iterator};

#[derive(Clone, PartialEq, Eq)]
pub struct RangeSet<T: RangeNumber> {
    ranges: Vec<RangeInc<T>>,
}

impl<T: RangeNumber> RangeSet<T> {
    pub fn new() -> Self {
        RangeSet { ranges: Vec::new() }
    }

    pub fn new_from_range(range: RangeInc<T>) -> Self {
        RangeSet {
            ranges: vec![range],
        }
    }

    pub fn all() -> Self {
        Self::new_from_range(RangeInc::<T>::all())
    }

    #[allow(unused)]
    pub fn is_empty(&self) -> bool {
        self.ranges.is_empty()
    }

    pub fn count(&self) -> T {
        let mut result = T::zero();

        for range in self.ranges.iter()
        {
            result += (range.end - range.start) + T::one();
        }

        result
    }

    #[allow(unused)]
    pub fn contains_value(&self, value: T) -> bool {
        self.ranges.iter().any(|r| r.contains(value))
    }

    pub fn ranges(&self) -> impl Iterator<Item = RangeInc<T>> {
        self.ranges.clone().into_iter()
    }

    pub fn values(&self) -> impl Iterator<Item = T> {
        self.ranges.clone().into_iter().map(|r| r.iter()).flatten()
    }

    #[allow(unused)]
    pub fn remove_value(&mut self, value: T) {
        self.remove_range(RangeInc::new_range(value, value));
    }

    pub fn remove_range(&mut self, range: RangeInc<T>) {
        self.ranges = self
            .ranges
            .iter()
            .map(|r| r.difference(range).into_iter())
            .flatten()
            .collect_vec();
    }

    pub fn insert_value(&mut self, value: T) {
        self.insert_range(RangeInc::new_range(value, value))
    }

    pub fn insert_range(&mut self, range: RangeInc<T>) {
        self.ranges.push(range);
        self.insert_fixup();
    }

    #[allow(unused)]
    pub fn insert_set(&mut self, set: &RangeSet<T>) {
        for range in set.ranges.iter() {
            self.insert_range(*range)
        }
    }

    pub fn insersection_with_range(mut self, range: RangeInc<T>) -> Self {
        self.insersect_with_range(range);
        self
    }

    pub fn insersect_with_range(&mut self, range: RangeInc<T>) {
        self.ranges = self
            .ranges
            .iter()
            .map(|r| r.intersection(range).into_iter())
            .flatten()
            .collect_vec()
    }

    pub fn inverse(mut self) -> Self {
        self.invert();
        self
    }

    pub fn invert(&mut self) {
        if self.ranges.is_empty() {
            self.ranges.push(RangeInc::all());
        } else
        // not empty
        {
            let cur_len = self.ranges.len();

            let mut new = Vec::new();
            new.reserve(cur_len + 2);

            if self.ranges[0].start > T::min_value() {
                new.push(RangeInc::new_range(
                    T::min_value(),
                    self.ranges[0].start - T::one(),
                ));
            }

            if self.ranges.len() >= 2 {
                for i in 0..(cur_len - 1) {
                    new.push(RangeInc::new_range(
                        self.ranges[i].end + T::one(),
                        self.ranges[i + 1].start - T::one(),
                    ));
                }
            }

            if self.ranges[cur_len - 1].end < T::max_value() {
                new.push(RangeInc::new_range(
                    self.ranges[cur_len - 1].end + T::one(),
                    T::max_value(),
                ));
            }

            self.ranges = new;
        }
    }

    fn insert_fixup(&mut self) {
        let mut new = self.ranges.drain(..).collect_vec();
        self.ranges.clear();

        new.sort_by(|a, b| a.start.cmp(&b.start));

        for r in new.into_iter() {
            match self.ranges.pop() {
                None => self.ranges.push(r),
                Some(o) => {
                    for ur in r.union(o) {
                        self.ranges.push(ur);
                    }
                }
            }
        }
    }
}

impl<T: RangeNumber> Debug for RangeSet<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_list().entries(self.ranges.iter()).finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::support::scan;

    fn test_set<T: 'static + RangeNumber + std::str::FromStr>(set: &RangeSet<T>, expected: &str)
    where
        T::Err: 'static + std::fmt::Debug,
    {
        let ranges = expected
            .split(",")
            .map(|rs| scan(rs).until("..=").parse::<T>().remaining().parse::<T>())
            .map(|(start, end)| RangeInc::new_range(start, end))
            .collect_vec();

        assert_eq!(ranges, set.ranges().collect_vec());
        assert_eq!(
            ranges.into_iter().flatten().collect_vec(),
            set.values().collect_vec()
        );
        assert_eq!(
            format!("[{}]", expected).replace(",", ", "),
            format!("{:?}", set)
        );
    }

    #[test]
    fn test_range_set() {
        let mut set = RangeSet::<u8>::all();

        test_set(&set, "0..=255");

        set.remove_value(0);
        test_set(&set, "1..=255");

        set.remove_value(1);
        test_set(&set, "2..=255");

        set.remove_value(0);
        test_set(&set, "2..=255");

        set.remove_value(255);
        test_set(&set, "2..=254");

        set.remove_value(254);
        test_set(&set, "2..=253");

        set.remove_value(255);
        test_set(&set, "2..=253");

        set.remove_value(100);
        test_set(&set, "2..=99,101..=253");

        set.remove_value(99);
        test_set(&set, "2..=98,101..=253");

        set.remove_value(101);
        test_set(&set, "2..=98,102..=253");

        set.remove_value(100);
        test_set(&set, "2..=98,102..=253");

        set.remove_value(150);
        test_set(&set, "2..=98,102..=149,151..=253");

        set.insert_value(160);
        test_set(&set, "2..=98,102..=149,151..=253");

        set.invert();
        test_set(&set, "0..=1,99..=101,150..=150,254..=255");

        set.invert();
        test_set(&set, "2..=98,102..=149,151..=253");

        set.insert_value(255);
        test_set(&set, "2..=98,102..=149,151..=253,255..=255");

        set.insert_value(254);
        test_set(&set, "2..=98,102..=149,151..=255");

        set.insert_value(1);
        test_set(&set, "1..=98,102..=149,151..=255");

        set.insert_value(0);
        test_set(&set, "0..=98,102..=149,151..=255");

        set.insert_value(150);
        test_set(&set, "0..=98,102..=255");

        set.insert_range(RangeInc::new_range(50, 200));
        test_set(&set, "0..=255");

        set.remove_value(30);
        test_set(&set, "0..=29,31..=255");

        set.insersect_with_range(RangeInc::new_range(20, 50));
        test_set(&set, "20..=29,31..=50");
    }
}
