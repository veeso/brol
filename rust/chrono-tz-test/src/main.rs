use chrono::{Local, NaiveDate, TimeZone, Utc};
use chrono_tz::Europe::Rome;

fn main() {
    let current_date = Utc::now().with_timezone(&Rome).date();
    println!("CURRENT DATE: {}", current_date);
    let other_date = NaiveDate::from_ymd(2022, 09, 01);
    let other_date = Rome.from_local_date(&other_date).unwrap();
    println!("OTHER DATE: {}", other_date);

    println!("CMP : {}", current_date >= other_date);
}
