use crate::time_table::time_h::TimeH;
use crate::time_table::activity::{ActIncludeStatus, ActInd, Activity};

use crate::tests::activity::{InterStatus, status_to_string};

#[test]
fn equal_intersect(){
    let mut act1 = Activity::new(
        "equals".to_string(), 
        TimeH::new(11, 44), 
        TimeH::new(15, 18)
    );

    let act2 = Activity::new(
        "equals".to_string(), 
        TimeH::new(11, 44), 
        TimeH::new(15, 18)
    );

    let act_res = act1.intersect_with(&act2); // equal activities

    let expected_act = Activity::new(
        "equals".to_string(), 
        TimeH::new(11, 44), 
        TimeH::new(15, 18)
    );

    match act_res { 
        Ok(act_qqch) => {
            assert_eq!(act1, expected_act)
        },
        _ => 
            panic!("Got intersection error while intersecting!")
    }
}

#[test]
fn no_intersect(){
    let mut act1 = Activity::new(
        "equals".to_string(), 
        TimeH::new(11, 44), 
        TimeH::new(15, 18)
    );

    let act3 = Activity::new(
        "equals".to_string(), 
        TimeH::new(15, 18),
        TimeH::new(16, 44), 
    );

    let act2 = Activity::new(
        "equals".to_string(), 
        TimeH::new(17, 44), 
        TimeH::new(19, 18)
    );

    let act_res = act1.intersect_with(&act2); // 
    match act_res {
        Err(_) => {},
        _ => { panic!("We expect to have an NoIntersection error here") }
    }
    
}

#[test]
fn included_intersect(){
    let mut act1 = Activity::new(
        "included1".to_string(), 
        TimeH::new(12, 15), 
        TimeH::new(14, 7)
    );

    let mut act2 = Activity::new(
        "included2".to_string(), 
        TimeH::new(11, 44), 
        TimeH::new(15, 18)
    );

    let act1_c = act1.clone();

    let rel1 = act1.intersect_with(&act2); // equal activities
    let rel2 = act2.intersect_with(&act1_c); // equal activities

    let mut act_exp = Activity::new(
        "included1".to_string(),
        TimeH::new(11, 44),
        TimeH::new(15, 18)
    );

    match rel1 { 
        Err(_) => { panic!("Got an intersection error whereas activities are included!") },
        _ => assert_eq!(act1, act_exp)
    }
    act_exp.title = "included2".to_string();
    match rel2 { 
        Err(_) => { panic!("Got an intersection error whereas activities are included!") },
        _ => assert_eq!(act2, act_exp)
    }

}

#[test]
fn included_start(){
    let mut act1 = Activity::new(
        "included1".to_string(), 
        TimeH::new(12, 15), 
        TimeH::new(14, 7)
    );

    let act2 = Activity::new(
        "included2".to_string(), 
        TimeH::new(13, 44), 
        TimeH::new(15, 18)
    );

    let act_expected = Activity::new("included1".to_string(), TimeH::new(12, 15), TimeH::new(15, 18));

    let rel1 = act1.intersect_with(&act2); // included end
    match rel1 { 
        Err(_) => { panic!("There's an error with intersecting activities") },
        _ => 
            assert_eq!(act1, act_expected)
    }
}

#[test]
fn included_end(){
    let mut act1 = Activity::new(
        "included1".to_string(), 
        TimeH::new(12, 15), 
        TimeH::new(14, 7)
    );

    let act2 = Activity::new(
        "included2".to_string(), 
        TimeH::new(13, 44), 
        TimeH::new(15, 18)
    );

    let act_res = act1.intersect_with(&act2); // included end

    let expected_act = Activity::new(
        "included1".to_string(), 
        TimeH::new(12, 15), 
        TimeH::new(15, 18));

    match act_res { 
        Err(_) => {panic!("Got error while intersection two activities! ")},
        _ => assert_eq!(act1, expected_act)
    }
}