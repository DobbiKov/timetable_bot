use crate::time_table::time_h::TimeH;


#[test]
fn time_h_creating(){
    let create_timeh = TimeH::new(23, 40);
    assert_eq!(create_timeh.get_time(), (23, 40))
}
#[test]
#[should_panic]
fn time_h_wrong_time(){
    TimeH::new(24, 11);
    TimeH::new(2, 60);
}

#[test]
fn time_h_to_string(){
    let timeh = TimeH::new(23,40);
    assert_eq!(timeh.to_string(), "23:40".to_string())
}

#[test]
fn time_h_addition(){
    let t1 = TimeH::new(11, 44);
    let t2 = TimeH::new(7, 51);
    
    assert_eq!(t1 + t2, TimeH::new(19, 35));
    assert_eq!(t1 + 62, TimeH::new(12,46));
}

#[test]
#[should_panic]
fn time_h_wrong_addition(){
    let t1 = TimeH::new(11, 44);
    let t2 = TimeH::new(13, 51);
    
    let res1 = t1 + t2;
}

#[test]
#[should_panic]
fn time_h_wrong_u32_addition(){
    let t1 = TimeH::new(11, 44);
    let t2 = TimeH::new(13, 51);
    
    let res2 = t1 + 10000;
}

#[test]
fn time_h_eq_operator(){
    let t1 = TimeH::new(11, 44);
    let t2 = TimeH::new(7, 51);
    let t3 = TimeH::new(11, 44);

    assert!(t1 != t2);
    assert_ne!(t1, t2);
    assert_eq!(t1, t3);

}

#[test]
fn time_h_ineq_operators(){
    let t1 = TimeH::new(11, 44);
    let t2 = TimeH::new(7, 51);
    let t3 = TimeH::new(11, 44);

    assert!(t1 > t2);
    assert!(t1 >= t2);
    assert!(t2 < t1);
    assert!(t2 <= t1);
    assert!(t1 >= t3);
    assert!(t1 <= t3);

}