use core::fmt;
use std::cmp;
use std::ops;

#[derive(Debug)]
pub struct TimeH {
    hour: u8,
    minute: u8,
}

impl Clone for TimeH {
    fn clone(&self) -> TimeH {
        *self
    }
}
impl Copy for TimeH {}

impl TimeH {
    pub fn new(hour: u8, minute: u8) -> TimeH {
        if !(0..=23).contains(&hour) || !(0..=59).contains(&minute) {
            panic!("Wrong hour or minute!");
        }

        TimeH { hour, minute }
    }
    pub fn get_time(&self) -> (u8, u8) {
        (self.hour, self.minute)
    }

    pub fn print(&self) {
        println!("{}:{:0}", self.hour, self.minute);
        ()
    }
}

impl fmt::Display for TimeH {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{:0}", self.hour, self.minute)
    }
}

impl ops::Add for TimeH {
    type Output = TimeH;

    fn add(self, other: TimeH) -> TimeH {
        let mut ft = self.get_time();

        let st = other.get_time();

        ft.0 += st.0;
        ft.1 += st.1;
        while ft.1 > 60 {
            ft.0 += 1;
            ft.1 -= 60;
        }
        if ft.0 >= 24 {
            panic!("Incorrect addition of TimeH");
        }

        TimeH {
            hour: ft.0,
            minute: ft.1,
        }
    }
}

impl ops::Add<u32> for TimeH {
    type Output = TimeH;

    fn add(self, other: u32) -> TimeH {
        let ft = self.get_time();

        let all_minutes = ft.1 as u32 + other;

        let mut hour: u8 = ft.0;
        let minute: u8 = (all_minutes % 60) as u8;
        if all_minutes >= 60 {
            hour += ((all_minutes - (minute as u32)) / 60) as u8;
        }

        if hour >= 24 {
            panic!("Incorrect addition of TimeH");
        }

        TimeH {
            hour: hour,
            minute: minute,
        }
    }
}

impl Eq for TimeH {}

impl PartialEq for TimeH {
    fn eq(&self, other: &TimeH) -> bool {
        let f1 = self.get_time();
        let f2 = other.get_time();

        f1.0 == f2.0 && f1.1 == f2.1
    }
}

impl PartialOrd for TimeH {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match self.hour.partial_cmp(&other.hour) {
            Some(core::cmp::Ordering::Equal) => {}
            ord => return ord,
        }
        self.minute.partial_cmp(&other.minute)
    }
}
