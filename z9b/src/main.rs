

enum Month{
    January,
    February,
    March,
    April,
    May,
    June,
    July,
    August,
    September,
    October,
    November,
    December
}


struct Date {
    day: u8,
    month: u8,
    year: u16,
    time: Option<Time>
}

use std::cmp::Ordering;
use std::str::FromStr;
impl Date {
    fn to_string(&self) -> String {
        if let Some(t) = &self.time {
            format!("{}-{}-{} {}:{}:{}", self.year, self.month,
                    self.day, t.hh, t.mm, t.ss)
        } else {
            format!("{}-{}-{}", self.year, self.month, self.day)
        }
    }


    fn from_3(day: u8, month: Month, year: u16) -> Date {
        Date{day,month:(month as u8 + 1), year, time:None}
    }


    fn from_string(string: &str, delim: char) -> Date {
        let mut s = string.split(delim);
        let day = u8::from_str(s.next().unwrap()).unwrap();
        let month = u8::from_str(s.next().unwrap()).unwrap();
        let year = u16::from_str(s.next().unwrap()).unwrap();
        Date{day,month,year, time:None}
    }

    fn has_time(&self) -> bool{
        self.time.is_some()
    }
    fn set_time(&mut self, time: Time){

    }
    fn clear_time(&mut self){

    }
}

struct Time {
    hh: u8,
    mm: u8,
    ss: u8
}

impl PartialEq for Time {
    fn eq(&self, time: &Time) -> bool {
        self.hh == time.hh && self.mm == time.mm && self.ss == time.ss
    }
}

impl Eq for Time {}

impl PartialOrd for Time {
    fn partial_cmp(&self, time: &Time) -> Option<std::cmp::Ordering> {
        let t1 = self.hh as u32 * 3600  +self.ss as u32 *60 + self.ss as u32;
        let t2 = time.hh as u32 *3600 + time.mm as u32 *60 + time.ss as u32;

        if t1 == t2 {
            Some(std::cmp::Ordering::Equal)
        } else if t1 < t2 {
            Some(std::cmp::Ordering::Less)
        } else {
            Some(std::cmp::Ordering::Greater)
        }

    }
}

impl Ord for Time {
    fn cmp(&self, time: &Time) -> std::cmp::Ordering {
        self.partial_cmp(time).unwrap()
    }
}
impl Time {
    fn to_string(&self) -> String {
        format!("{}-{}-{}", self.hh, self.mm, self.ss)
    }
    fn from_3(hh: u8, mm: u8, ss: u8) -> Time {
        let mm = mm + ss / 60;
        let ss = ss % 60;
        let hh = hh + mm / 60;
        let mm = mm % 60;
        let hh = hh % 24;
        Time{hh,mm,ss}
    }
    fn from_string(string: &str, delim: char) -> Time {
        let mut s = string.split(delim);
        let hh = u8::from_str(s.next().unwrap()).unwrap();
        let mm = u8::from_str(s.next().unwrap()).unwrap();
        let ss = u8::from_str(s.next().unwrap()).unwrap();
        Time{hh,mm,ss}
    }
}



fn main() {
    let d1 = Date::from_3(11, Month::November, 2011);
    println!("{}", d1.to_string());
    let d2 = Date::from_string("13/5/2022", '/');
    println!("{}", d2.to_string());

    let t1 = Time::from_string("14:34:45", ':');
    println!("{}", t1.to_string());


    


}
