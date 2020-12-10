pub struct Answer<T, U>
{
    pub calculated: T,
    pub expected: U,
}

pub struct StrAnswer
{
    pub calculated: String,
    pub expected: String,
}

impl<T: ToString, U: ToString> Into<StrAnswer> for Answer<T, U>
{
    fn into(self) -> StrAnswer
    {
        StrAnswer
        {
            calculated: self.calculated.to_string(),
            expected: self.expected.to_string(),
        }
    }
}

