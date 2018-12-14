extern crate chrono;
use chrono::{DateTime, TimeZone, Utc};

// Returns a Utc DateTime one billion seconds after start.
pub fn after(start: DateTime<Utc>) -> DateTime<Utc> {
  let epoch: i64 = start.timestamp();
  let plus_giga: i64 = epoch + 1_000_000_000;
  let time: DateTime<Utc> = Utc.timestamp(plus_giga, 0);
  return time;
}
