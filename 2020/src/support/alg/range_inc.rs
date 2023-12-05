use std::fmt::Debug;
use std::hash::Hash;
use num_traits::{One, Bounded, NumOps, NumAssignOps};

pub trait RangeNumber: Sized + Clone + Copy + Debug + Bounded + One + NumOps + NumAssignOps + PartialEq + Eq + PartialOrd + Ord
{
}

impl<T: Sized + Clone + Copy + Debug + Bounded + One + NumOps + NumAssignOps + PartialEq + Eq + PartialOrd + Ord> RangeNumber for T
{
}

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
pub struct RangeInc<T: RangeNumber>
{
    pub start: T,
    pub end: T,
}

impl<T: RangeNumber> RangeInc<T>
{
    pub fn new_range(start: T, end: T) -> Self
    {
        if end < start { panic!("RangeInc start must be <= end"); }
        RangeInc { start, end }
    }
    
    pub fn all() -> Self
    {
        RangeInc { start: T::min_value(), end: T::max_value() }
    }

    pub fn iter(&self) -> RangeIncIter<T>
    {
        RangeIncIter { start: self.start, end: self.end, finished: false }
    }

    pub fn extent(&self, range: RangeInc<T>) -> RangeInc<T>
    {
        RangeInc::new_range(self.start.min(range.start), self.end.max(range.end))
    }

    pub fn difference(&self, range: RangeInc<T>) -> RangeIncPairIter<T>
    {
        if range.end < self.start
        {
            RangeIncPairIter::one(*self)
        }
        else if range.start <= self.start && range.end >= self.end
        {
            RangeIncPairIter::none()
        }
        else if range.start <= self.start && range.end >= self.start && range.end < self.end
        {
            RangeIncPairIter::one(RangeInc::new_range(range.end + T::one(), self.end))
        }
        else if range.start > self.start && range.end < self.end
        {
            RangeIncPairIter::two(
                RangeInc::new_range(self.start, range.start - T::one()),
                RangeInc::new_range(range.end + T::one(), self.end))
        }
        else if self.start < range.start && range.start <= self.end && range.end >= self.end
        {
            RangeIncPairIter::one(RangeInc::new_range(self.start, range.start - T::one()))
        }
        else // past end
        {
            assert!(range.start > self.end);
            RangeIncPairIter::one(*self)
        }
    }

    pub fn union(&self, range: RangeInc<T>) -> RangeIncPairIter<T>
    {
        if range.end < self.start && (range.end + T::one()) == self.start
        {
            RangeIncPairIter::one(RangeInc::new_range(range.start, self.end))
        }
        else if range.end < self.start
        {
            RangeIncPairIter::two(range, *self)
        }
        else if range.start <= self.end && self.start <= range.end
        {
            RangeIncPairIter::one(RangeInc::new_range(
                self.start.min(range.start),
                self.end.max(range.end)))
        }
        else if self.end < range.start && (self.end + T::one()) == range.start
        {
            RangeIncPairIter::one(RangeInc::new_range(self.start, range.end))
        }
        else
        {
            assert!(self.end < range.start);
            RangeIncPairIter::two(*self, range)
        }
    }

    pub fn intersection(&self, range: RangeInc<T>) -> Option<RangeInc<T>>
    {
        if range.end < self.start
        {
            None
        }
        else if range.start <= self.end && self.start <= range.end
        {
            Some(RangeInc::new_range(
                self.start.max(range.start),
                self.end.min(range.end)))
        }
        else
        {
            assert!(self.end < range.start);
            None
        }
    }

    pub fn contains(&self, value: T) -> bool
    {
        self.start <= value && value <= self.end
    }
}

impl<T: RangeNumber> Debug for RangeInc<T>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
    {
        f.write_str(&format!("{:?}..={:?}", self.start, self.end))
    }
}

impl<T: RangeNumber> IntoIterator for RangeInc<T>
{
    type Item = T;
    type IntoIter = RangeIncIter<T>;

    fn into_iter(self) -> Self::IntoIter
    {
        RangeIncIter { start: self.start, end: self.end, finished: false }
    }
}

pub struct RangeIncIter<T: RangeNumber>
{
    start: T,
    end: T,
    finished: bool,
}

impl<T: RangeNumber> Iterator for RangeIncIter<T>
{
    type Item = T;
    fn next(&mut self) -> Option<T>
    {
        if self.finished
        {
            None
        }
        else
        {
            let result = Some(self.start);

            if self.start == self.end
            {
                self.finished = true;
            }
            else
            {
                self.start += T::one();
            }

            result
        }
    }
}

pub struct RangeIncPairIter<T: RangeNumber>
{
    index: usize,
    num: usize,
    items: [RangeInc<T>; 2],
}

impl<T: RangeNumber> RangeIncPairIter<T>
{
    fn none() -> Self
    {
        RangeIncPairIter{ index: 0, num: 0, items: [RangeInc::all(), RangeInc::all()], }
    }

    fn one(a: RangeInc<T>) -> Self
    {
        RangeIncPairIter{ index: 0, num: 1, items: [a, RangeInc::all()], }
    }

    fn two(a: RangeInc<T>, b: RangeInc<T>) -> Self
    {
        RangeIncPairIter{ index: 0, num: 2, items: [a, b], }
    }
}

impl<T: RangeNumber> Iterator for RangeIncPairIter<T>
{
    type Item = RangeInc<T>;
    fn next(&mut self) -> Option<RangeInc<T>>
    {
        if self.index >= self.num
        {
            None
        }
        else
        {
            self.index += 1;
            Some(self.items[self.index - 1])
        }
    }
}
