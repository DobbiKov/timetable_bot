pub mod time_h;
pub mod activity;

use std::ops::Deref;

use time_h::{TimeH};
use activity::{Activity};

pub struct TimeTable {
    pub title: String,
    activities: Vec<Activity>,
}

impl TimeTable{
    pub fn new(title: String, activities: Vec<Activity>) -> TimeTable{
        TimeTable{
            title: title,
            activities: activities
        }
    }
    pub fn get_activities(&self) -> &Vec<Activity>{
        &self.activities
    }

    pub fn print(&self){
        println!("** {}", self.title);
        for act in self.activities.deref().into_iter(){
            act.print()
        }
    }
    
}




