pub mod activity;
pub mod time_h;

use std::collections::LinkedList;
use std::mem;
use std::ops::Deref;

use activity::{starts_ealrier, starts_ealrier_ind, Activity, NoIntersectionError};
use time_h::TimeH;

pub struct TimeTable {
    pub title: String,
    activities: Vec<Activity>,
}

impl TimeTable {
    pub fn new(title: String, activities: Vec<Activity>) -> TimeTable {
        TimeTable {
            title: title,
            activities: activities,
        }
    }
    pub fn empty() -> TimeTable {
        TimeTable {
            title: "".to_string(),
            activities: Vec::<Activity>::new(),
        }
    }
    pub fn inverse(&self) -> TimeTable {
        if self.activities.is_empty() {
            return TimeTable {
                title: self.title.clone(),
                activities: vec![Activity::new(
                    "".to_string(),
                    TimeH::new(0, 0),
                    TimeH::new(23, 59),
                )],
            };
        }
        let f_act = Activity::new(
            "".to_string(),
            TimeH::new(0, 0),
            TimeH::from(self.activities.first().unwrap().get_start()),
        );
        let acts_size = self.activities.len();
        let mut acts_res = vec![f_act];
        for act_ind in 1..acts_size {
            let act = (self.activities[act_ind]).clone();
            let prev_act = (self.activities[act_ind - 1]).clone();
            acts_res.push(Activity::new(
                "".to_string(),
                TimeH::from(prev_act.get_end()),
                TimeH::from(act.get_start()),
            ))
        }
        let last_act_time = acts_res.last().unwrap().get_end();

        if last_act_time.get_time() != (23, 59) {
            let last_act = self.activities.last().unwrap();
            acts_res.push(Activity::new(
                "".to_string(),
                TimeH::from(last_act.get_end()),
                TimeH::new(23, 59),
            ));
        }

        TimeTable::new("inverse".to_string(), acts_res)
    }
    pub fn get_activities(&self) -> &Vec<Activity> {
        &self.activities
    }

    pub fn print(&self) {
        println!("** {}", self.title);
        for act in self.activities.deref().into_iter() {
            act.print()
        }
    }
}

impl Clone for TimeTable {
    fn clone(&self) -> Self {
        Self {
            title: self.title.clone(),
            activities: self.activities.clone(),
        }
    }
}

pub fn intersect_time_tables(t1: &TimeTable, t2: &TimeTable) -> TimeTable {
    let mut acts_res = Vec::<Activity>::new();
    let acts1 = t1.get_activities().clone();
    let acts2 = t2.get_activities().clone();

    // TODO
    let mut iter1 = acts1.iter();
    let mut iter2 = acts2.iter();
    let mut act1_o = iter1.next();
    let mut act2_o = iter2.next();
    while act1_o.is_some() || act2_o.is_some() {
        //intersect two activities
        if act1_o.is_none() {
            //if act1_0 it means that there's no more activities in first arr,
            //there're probablu activities in the second arr, we swap them to work
            //with second arr as it would be first
            //as we swapped them, second arr will be definitely empty
            mem::swap(&mut act1_o, &mut act2_o);
            mem::swap(&mut iter1, &mut iter2);
        }
        if act2_o.is_none() {
            //second array is empty then first isn't we try to intersect the first
            //activity with the last one in result
            match acts_res
                .last_mut()
                .unwrap()
                .intersect_with(&(act1_o.unwrap().clone()))
            {
                Ok(_) => {}
                Err(_) => {
                    // if we can't intersect them, we push the activity to the array
                    acts_res.push(act1_o.unwrap().clone());
                }
            }
            act1_o = iter1.next();
            continue;
        }
        let mut act1 = act1_o.unwrap().clone();
        let mut act2 = act2_o.unwrap().clone();
        if acts_res.is_empty() {
            //if res array is empty, then we push the activity that starts starts ealrier
            acts_res.push(starts_ealrier(&act1, &act2));
            continue;
        }
        if starts_ealrier_ind(&act1, &act2) == 2 {
            //if the second activity starts earlier, we swap
            //them to work with the second as with first
            mem::swap(&mut act1, &mut act2);
            mem::swap(&mut act1_o, &mut act2_o);
            mem::swap(&mut iter1, &mut iter2);
        }
        match acts_res.last_mut().unwrap().intersect_with(&act1) {
            //if we can intersect then the
            //result of intersection will be
            //in the last element of res
            //array
            Ok(_) => {
                act1_o = iter1.next();
                continue;
            }
            Err(NoIntersectionError) => {
                // if we can't intersect them we push the activity to the
                // array
                acts_res.push(act1);
                act1_o = iter1.next();
                continue;
            }
        }
    }
    TimeTable::new(t1.title.clone(), acts_res)
}
