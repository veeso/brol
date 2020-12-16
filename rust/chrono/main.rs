/**
 *
 *
 *           DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
 *                   Version 2, December 2004
 *
 *  Copyright (C) 2020 Christian Visintin
 *
 *  Everyone is permitted to copy and distribute verbatim or modified
 *  copies of this license document, and changing it is allowed as long
 *  as the name is changed.
 *
 *             DO WHAT THE FUCK YOU WANT TO PUBLIC LICENSE
 *    TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION AND MODIFICATION
 *
 *   0. You just DO WHAT THE FUCK YOU WANT TO.
*/

extern crate chrono;

use chrono::format::ParseError;
use chrono::prelude::*;
use std::time::{Duration, SystemTime};

/// ### instant_to_str
///
/// Format a `Instant` into a time string
fn time_to_str(time: SystemTime, fmt: &str) -> String {
    let datetime: DateTime<Local> = time.into();
    format!("{}", datetime.format(fmt))
}

/// ### lstime_to_systime
///
/// Convert ls syntax time to System Time
/// ls time has two possible syntax:
/// 1. if year is current: %b %d %H:%M (e.g. Nov 5 13:46)
/// 2. else: %b %d %Y (e.g. Nov 5 2019)
fn lstime_to_systime(
    tm: &str,
    fmt_year: &str,
    fmt_hours: &str,
) -> Result<SystemTime, ParseError> {
    let datetime: NaiveDateTime = match NaiveDate::parse_from_str(tm, fmt_year) {
        Ok(date) => {
            // Case 2.
            // Return NaiveDateTime from NaiveDate with time 00:00:00
            date.and_hms(0, 0, 0)
        }
        Err(_) => {
            // Might be case 1.
            // We need to add Current Year at the end of the string
            let this_year: i32 = Utc::now().year();
            let date_time_str: String = format!("{} {}", tm, this_year);
            // Now parse
            match NaiveDateTime::parse_from_str(
                date_time_str.as_ref(),
                format!("{} %Y", fmt_hours).as_ref(),
            ) {
                Ok(dt) => dt,
                Err(err) => return Err(err),
            }
        }
    };
    // Convert datetime to system time
    let sys_time: SystemTime = SystemTime::UNIX_EPOCH;
    Ok(sys_time
        .checked_add(Duration::from_secs(datetime.timestamp() as u64))
        .unwrap_or(SystemTime::UNIX_EPOCH))
}

fn main() {
    let time: SystemTime = SystemTime::UNIX_EPOCH;
    // Set time to 2020-12-16T18:43:40+0100
    let time: SystemTime = time.checked_add(Duration::from_secs(1608140620)).unwrap();
    // Iso 8601
    println!(time_to_str(time, "%Y-%m-%dT%H:%M:%S%Z"));
    // ls times
    assert_eq!(
        lstime_to_systime("Nov 5 16:32", "%b %d %Y", "%b %d %H:%M")
            .ok()
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()
            .unwrap(),
        Duration::from_secs(1604593920)
    );
    assert_eq!(
        lstime_to_systime("Dec 2 21:32", "%b %d %Y", "%b %d %H:%M")
            .ok()
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()
            .unwrap(),
        Duration::from_secs(1606944720)
    );
    assert_eq!(
        lstime_to_systime("Nov 5 2018", "%b %d %Y", "%b %d %H:%M")
            .ok()
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()
            .unwrap(),
        Duration::from_secs(1541376000)
    );
    assert_eq!(
        lstime_to_systime("Mar 18 2018", "%b %d %Y", "%b %d %H:%M")
            .ok()
            .unwrap()
            .duration_since(SystemTime::UNIX_EPOCH)
            .ok()
            .unwrap(),
        Duration::from_secs(1521331200)
    );
}
