use std::{str::FromStr, string::ParseError};

struct Assignment {
    start: u32,
    end: u32
}

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        return self.start <= other.start && self.end >= other.end;
    }

    fn overlaps_at_all(&self, other: &Assignment) -> bool {
        let check_1 = other.start >= self.start && other.start <= self.end;
        let check_2 = other.end >= self.start && other.end <= self.end;
        return check_1 || check_2;
    }
}

impl FromStr for Assignment {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once("-").unwrap();
        let left_num: u32 = left.parse().unwrap();
        let right_num: u32 = right.parse().unwrap();

        return Ok(Assignment { start: left_num, end: right_num });
    }
}

pub struct Group {
    one: Assignment,
    two: Assignment
}

impl Group {
    pub fn needs_reconsidering(&self) -> bool {
        return self.one.contains(&self.two) || self.two.contains(&self.one);
    }

    pub fn contains_overlaps(&self) -> bool {
        return self.one.overlaps_at_all(&self.two) || self.two.overlaps_at_all(&self.one);
    }
}

impl FromStr for Group {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(",").unwrap();
        return Ok(Group { 
            one: Assignment::from_str(left).unwrap(), 
            two: Assignment:: from_str(right).unwrap() 
        })
    }
}