use crate::time_table::time_h::TimeH;
use crate::time_table::activity::{ActIncludeStatus, ActInd, Activity};

use crate::tests::activity::{InterStatus, status_to_string};
// intersection 


#[test]
fn equal_intersection(){
    let act1 = Activity::new(
        "equals".to_string(), 
        TimeH::new(11, 44), 
        TimeH::new(15, 18)
    );

    let act2 = Activity::new(
        "equals".to_string(), 
        TimeH::new(11, 44), 
        TimeH::new(15, 18)
    );

    let rel1 = act1.get_relation(&act2); // equal activities
    match rel1 { 
        (ActInd::None, ActIncludeStatus::Equals) => {},
        _ => 
            panic!("The relation of two equal activities is wrong! State: {}, Expected: {}", 
            status_to_string(rel1),
            status_to_string((ActInd::None, ActIncludeStatus::Equals)) )
    }
}

#[test]
fn no_intersection(){
    let act1 = Activity::new(
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

    let rel1 = act1.get_relation(&act2); // 
    match rel1 { 
        (ActInd::None, ActIncludeStatus::NoIntersection) => {},
        _ => 
            panic!("The relation of two activities is wrong! State: {}, Expected: {}", 
            status_to_string(rel1),
            status_to_string((ActInd::None, ActIncludeStatus::NoIntersection)) )
    }

    let rel2 = act1.get_relation(&act3); // 
    match rel2 { 
        (ActInd::None, ActIncludeStatus::NoIntersection) => {
            panic!("The relation of two activities is wrong! State: {}, Expected: {}", 
            status_to_string(rel2),
            status_to_string((ActInd::First, ActIncludeStatus::IncludedEnd)) )
        },
        _ => {},
    }
}

#[test]
fn included_intersection(){
    let act1 = Activity::new(
        "included1".to_string(), 
        TimeH::new(12, 15), 
        TimeH::new(14, 7)
    );

    let act2 = Activity::new(
        "included2".to_string(), 
        TimeH::new(11, 44), 
        TimeH::new(15, 18)
    );

    let rel1 = act1.get_relation(&act2); // equal activities
    let rel2 = act2.get_relation(&act1); // equal activities
    match rel1 { 
        (ActInd::First, ActIncludeStatus::OneIncluded) => {},
        _ => 
            panic!("The relation of two activities is wrong! State: {}, Expected: {}", 
            status_to_string(rel1),
            status_to_string((ActInd::First, ActIncludeStatus::OneIncluded)) )
    }
    match rel2 { 
        (ActInd::Second, ActIncludeStatus::OneIncluded) => {},
        _ => 
            panic!("The relation of two activities is wrong! State: {}, Expected: {}", 
            status_to_string(rel2),
            status_to_string((ActInd::Second, ActIncludeStatus::OneIncluded)) )
    }
}

#[test]
fn included_start(){
    let act1 = Activity::new(
        "included1".to_string(), 
        TimeH::new(12, 15), 
        TimeH::new(14, 7)
    );

    let act2 = Activity::new(
        "included2".to_string(), 
        TimeH::new(13, 44), 
        TimeH::new(15, 18)
    );

    let rel1 = act2.get_relation(&act1); // included end
    match rel1 { 
        (ActInd::First, ActIncludeStatus::IncludedStart) => {},
        _ => 
            panic!("The relation of two activities is wrong! State: {}, Expected: {}", 
            status_to_string(rel1),
            status_to_string((ActInd::First, ActIncludeStatus::IncludedStart)) )
    }
}

#[test]
fn included_end(){
    let act1 = Activity::new(
        "included1".to_string(), 
        TimeH::new(12, 15), 
        TimeH::new(14, 7)
    );

    let act2 = Activity::new(
        "included2".to_string(), 
        TimeH::new(13, 44), 
        TimeH::new(15, 18)
    );

    let rel1 = act1.get_relation(&act2); // included end
    match rel1 { 
        (ActInd::First, ActIncludeStatus::IncludedEnd) => {},
        _ => 
            panic!("The relation of two activities is wrong! State: {}, Expected: {}", 
            status_to_string(rel1),
            status_to_string((ActInd::First, ActIncludeStatus::IncludedEnd)) )
    }
}