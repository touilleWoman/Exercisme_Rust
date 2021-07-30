use chrono::{DateTime, Duration, Utc};
// Returns a Utc DateTime one billion seconds after start.

pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
    const GIGA: i64 = 1_000_000_000;
    let after = start.checked_add_signed(Duration::seconds(GIGA));
    match after {
        Some(x) => {
            println!("{}", x);
            x
        }
        None => {
            panic!("Overflow");
        }
    }
}

// pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
//     start + Duration::seconds(1_000_000_000)
// }
