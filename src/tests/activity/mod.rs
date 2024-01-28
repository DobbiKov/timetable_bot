pub mod relation;
pub mod intersection;

use crate::time_table::time_h::TimeH;
use crate::time_table::activity::{ActIncludeStatus, ActInd, Activity};

type InterStatus = (ActInd, ActIncludeStatus);
fn status_to_string(statuses: InterStatus) -> String{

    format!("({}, {})", statuses.0.to_string(), statuses.1.to_string())
}

#[test]
fn create_activity(){
    let t1 = TimeH::new(14, 25);
    let t2 = TimeH::new(15, 11);

    let act = Activity::new("title".to_string(), t1, t2);
    assert_eq!(act.title, "title".to_string());
    assert_eq!(act.get_start(), t1);
    assert_eq!(act.get_end(), t2);
}

#[test]
#[should_panic]
fn create_activity_start_after_end(){
    let act = Activity::new(
        "title".to_string(), 
        TimeH::new(23, 14), 
        TimeH::new(11, 07)
    );
}

#[test]
#[should_panic]
fn create_activity_start_eq_end(){
    let act = Activity::new(
        "title".to_string(), 
        TimeH::new(23, 14), 
        TimeH::new(23, 14)
    );
}

#[test]
fn setters(){
    let mut act = Activity::new(
        "title".to_string(), 
        TimeH::new(14, 37), 
        TimeH::new(23, 14)
    );
    act.set_start(TimeH::new(15, 17));
    assert_eq!(act.get_start(), TimeH::new(15, 17));

    act.set_end(TimeH::new(17, 47));
    assert_eq!(act.get_end(), TimeH::new(17, 47));

}

#[test]
fn to_string(){
    let mut act = Activity::new(
        "title".to_string(), 
        TimeH::new(14, 37), 
        TimeH::new(23, 14)
    );
    
    assert_eq!(act.to_string(), "title: 14:37 - 23:14".to_string());

}