mod tests;
mod time_table;

use time_table::activity::Activity;
use time_table::intersect_time_tables;
use time_table::time_h::TimeH;

use crate::time_table::TimeTable;
fn main() {
    println!("Hello, world!");
    //let t: TimeH = TimeH::new(10, 57);
    //let t1: TimeH = TimeH::new(12, 43);
    //// t.print();
    //let mut act: Activity = Activity::new("Math".to_string(), t, t1);

    //let act2: Activity = Activity::new("English".to_string(),
    //TimeH::new(4, 33), TimeH::new(7, 55)
    //);

    //let time_table = TimeTable::new("Day".to_string(), vec![act2, act]);

    //time_table.print();

    let act1: Activity = Activity::new("Math".to_string(), TimeH::new(8, 30), TimeH::new(9, 45));
    let act2: Activity = Activity::new(
        "English".to_string(),
        TimeH::new(10, 00),
        TimeH::new(11, 15),
    );
    let act3: Activity = Activity::new(
        "Science".to_string(),
        TimeH::new(11, 30),
        TimeH::new(12, 45),
    );
    let act4: Activity = Activity::new(
        "History".to_string(),
        TimeH::new(13, 00),
        TimeH::new(14, 15),
    );
    let act5: Activity = Activity::new("Art".to_string(), TimeH::new(14, 30), TimeH::new(15, 45));

    let time_table = TimeTable::new("Day".to_string(), vec![act1, act2, act3, act4, act5]);

    let lesson1: Activity =
        Activity::new("Physics".to_string(), TimeH::new(9, 00), TimeH::new(10, 15));
    let lesson2: Activity = Activity::new(
        "Chemistry".to_string(),
        TimeH::new(10, 30),
        TimeH::new(11, 45),
    );
    let lesson4: Activity = Activity::new(
        "Geography".to_string(),
        TimeH::new(13, 30),
        TimeH::new(14, 45),
    );
    let lesson5: Activity =
        Activity::new("Music".to_string(), TimeH::new(15, 00), TimeH::new(16, 00));

    let schedule = TimeTable::new(
        "School Day".to_string(),
        vec![lesson1, lesson2, lesson4, lesson5],
    );

    let res = intersect_time_tables(&time_table, &schedule);
    res.print();
    println!("\n");
    res.inverse().print();
    // println!("{:?}", t.get_time());
}
