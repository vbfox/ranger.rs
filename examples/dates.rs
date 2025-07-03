use chrono::DateTime;
use chrono::Utc;
use range_ranger::Range;

fn main() {
    let start: DateTime<Utc> = "2000-01-01T00:00:00Z".parse().unwrap();
    let end: DateTime<Utc> = "2000-12-31T00:00:00Z".parse().unwrap();
    let r: Range<_> = (start..=end).into();

    println!("Range: {r:?}");
    let d1: DateTime<Utc> = "2000-06-15T00:00:00Z".parse().unwrap();
    println!("Contains {}: {}", d1, r.contains(d1));
    let d2: DateTime<Utc> = "2020-01-01T00:00:00Z".parse().unwrap();
    println!("Contains {}: {}", d2, r.contains(d2));
}
