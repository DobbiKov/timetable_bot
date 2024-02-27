use super::time_h::TimeH;
use std::cmp;

pub struct NoIntersectionError;

pub enum ActIncludeStatus{
    None,
    OneIncluded,
    IncludedStart,
    IncludedEnd,
    NoIntersection,
    Equals
}
impl ActIncludeStatus{
    pub fn to_string(&self) -> String{
        let str = match self{
            ActIncludeStatus::Equals => "ActIncludeStatus::Equals",
            ActIncludeStatus::OneIncluded => "ActIncludeStatus::OneIncluded",
            ActIncludeStatus::IncludedEnd => "ActIncludeStatus::IncludedEnd",
            ActIncludeStatus::IncludedStart => "ActIncludeStatus::IncludedStart",
            ActIncludeStatus::NoIntersection => "ActIncludeStatus::NoIntersection",
            ActIncludeStatus::None => "ActIncludeStatus::None",
        };

        String::from(str)
    }
}

pub enum ActInd {
    None,
    First,
    Second,
}

impl ActInd{
    pub fn to_string(&self) -> String{
        let str = match self{
            ActInd::None => "ActInd::None",
            ActInd::First => "ActInd::First",
            ActInd::Second => "ActInd::Second",
        };
        String::from(str)
    }
    pub fn print(&self){
        println!("{}", self.to_string())
    }
}

pub struct Activity {
    pub title: String,
    start: TimeH,
    end: TimeH,
}

impl Clone for Activity {
    fn clone(&self) -> Activity {
        Activity::new(self.title.clone(), self.start.clone(), self.end.clone())
    }
}

impl Activity {
    pub fn new(title: String, start: TimeH, end: TimeH) -> Activity {
        if start > end {
            panic!("Start must be before the end!")
        }
        if start == end {
            panic!("Start must be before the end (they can't be equal)!")
        }
        Activity { title, start, end }
    }
    pub fn intersect_with(&mut self, other: &Activity) -> Result<(), NoIntersectionError>{
        let rel = self.get_relation(other);
        match rel{
            (_, ActIncludeStatus::None) => {
                Err(NoIntersectionError)
            },
            (_, ActIncludeStatus::Equals) => {
                Ok(())
            },
            (_, ActIncludeStatus::NoIntersection) => {
                Err(NoIntersectionError)
            },
            (ActInd::None, _) => {
                Err(NoIntersectionError)
            },
            (ActInd::First, ActIncludeStatus::OneIncluded) => {
                self.set_start(other.start);
                self.set_end(other.end);
                Ok(())
            },
            (ActInd::First, ActIncludeStatus::IncludedStart) => {
                self.set_start(other.start);
                Ok(())
            },
            (ActInd::First, ActIncludeStatus::IncludedEnd) => {
                self.set_end(other.end);
                Ok(())
            },


            (ActInd::Second, ActIncludeStatus::OneIncluded) => {
                Ok(())
            },
            (ActInd::Second, ActIncludeStatus::IncludedStart) => {
                self.set_end(other.end);
                Ok(())
            },
            (ActInd::Second, ActIncludeStatus::IncludedEnd) => {
                self.set_start(other.start);
                Ok(())
            },

        }
    }
    pub fn get_start(&self) -> TimeH {
        self.start
    }
    pub fn get_end(&self) -> TimeH {
        self.end
    }
    pub fn set_start(&mut self, start: TimeH){
        self.start = start
    }
    pub fn set_end(&mut self, end: TimeH){
        self.end = end
    }
    pub fn to_string(&self) -> String{
        format!("{}: {} - {}", self.title, self.start.to_string(), self.end.to_string())
    }
    pub fn print(&self){
        println!("- {}", self.to_string())
    }
    pub fn get_relation(&self, other: &Activity) -> (ActInd, ActIncludeStatus){
        if self.start == other.start && self.end == other.end{
            return (ActInd::None, ActIncludeStatus::Equals);
        }
        if self.start <= other.start && self.end >= other.end{
            return (ActInd::Second, ActIncludeStatus::OneIncluded);
        }
        if self.start >= other.start && self.end <= other.end{
            return (ActInd::First, ActIncludeStatus::OneIncluded);
        }

        if self.start > other.end || self.end < other.start{
            return (ActInd::None, ActIncludeStatus::NoIntersection);
        }

        if self.start >= other.start && self.start < other.end{
            return (ActInd::First, ActIncludeStatus::IncludedStart);
        }
        if self.end >= other.start && self.end < other.end{
            return (ActInd::First, ActIncludeStatus::IncludedEnd);
        }

        if other.start >= self.start && other.start < self.end{
            return (ActInd::First, ActIncludeStatus::IncludedStart);
        }
        if other.end >= self.start && other.end < self.end{
            return (ActInd::Second, ActIncludeStatus::IncludedEnd);
        }

        return (ActInd::None, ActIncludeStatus::None);


    }
}