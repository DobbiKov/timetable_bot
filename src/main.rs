mod time_table;
mod tests;

use time_table::time_h::{TimeH};
use time_table::activity::{Activity};

use crate::time_table::{TimeTable};
fn main() {
    println!("Hello, world!");
    let t: TimeH = TimeH::new(10, 57);
    let t1: TimeH = TimeH::new(12, 43);
    // t.print();
    let mut act: Activity = Activity::new("Math".to_string(), t, t1);

    let act2: Activity = Activity::new("English".to_string(), 
    TimeH::new(4, 33), TimeH::new(7, 55)
    );

    let act3: Activity = Activity::new("English".to_string(), 
    t, t1
    );

    act.print();
    let res_inter = act.intersect_with(&act2);
    match res_inter {
        Ok(_) => { act.print() },
        Err(_) => { println!("There is no intersection!") }
    }
    

    let rel = act.get_relation(&act2);
    println!("{}", rel.0.to_string());
    println!("{}", rel.1.to_string());

    // act.print();

    // let time_table = TimeTable::new("Day".to_string(), vec![act, act2]);

    // time_table.print();
    println!("{:?}", t.get_time());
}
