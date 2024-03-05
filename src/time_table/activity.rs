//! # Activity
//! 
//! A module that represents an activity and can get intersection of two activities

use super::time_h::TimeH;
use std::fmt::Debug;
// use std::cmp;

pub struct NoIntersectionError;

/// An Enum that represents an IntersectionStatus 
/// 
/// ## Example:
/// If the end of an activity is included in the other activity(act 1: 11:44 - *17:57* | act 2: 12:46 - 19:44)
///
/// **ActIncludeStatus::IncludedEnd** represents this situation
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

/// An Enum that represents which activitie is related to status 
/// 
/// ## Example:
/// If the end of an activity is included in the other activity(act 1: 11:44 - *17:57* | act 2: 12:46 - 19:44)
///
/// **ActInd::First** represents this situation
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

/// Stuct that represents an acivity (title, start time and finish time)
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

impl Eq for Activity{}

impl Debug for Activity{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Activity").field("title", &self.title).field("start", &self.start).field("end", &self.end).finish()
    }
}

impl PartialEq for Activity{
    fn eq(&self, other: &Self) -> bool {
        self.title == other.title && self.start == other.start && self.end == other.end
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
        if self.get_start() == other.get_start() && self.get_end() == other.get_end(){
            return (ActInd::None, ActIncludeStatus::Equals);
        }
        if self.get_start() <= other.get_start() && self.get_end() >= other.get_end(){
            return (ActInd::Second, ActIncludeStatus::OneIncluded);
        }
        if self.get_start() >= other.get_start() && self.get_end() <= other.get_end(){
            return (ActInd::First, ActIncludeStatus::OneIncluded);
        }

        if self.get_start() > other.get_end() || self.get_end() < other.get_start(){
            return (ActInd::None, ActIncludeStatus::NoIntersection);
        }

        if self.get_start() >= other.get_start() && self.get_start() <= other.get_end(){
            return (ActInd::First, ActIncludeStatus::IncludedStart);
        }
        if self.get_end() >= other.get_start() && self.get_end() <= other.get_end(){
            return (ActInd::First, ActIncludeStatus::IncludedEnd);
        }

        if other.get_start() >= self.get_start() && other.get_start() < self.get_end(){
            return (ActInd::Second, ActIncludeStatus::IncludedStart);
        }
        if other.get_end() >= self.get_start() && other.get_end() < self.get_end(){
            return (ActInd::Second, ActIncludeStatus::IncludedEnd);
        }

        return (ActInd::None, ActIncludeStatus::None);


    }
}