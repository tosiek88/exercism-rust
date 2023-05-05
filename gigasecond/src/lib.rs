use time::{ext::NumericalDuration, Date, PrimitiveDateTime as DateTime};
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    start
        .checked_add((365 * 31 + 259).days())
        .unwrap()
        .checked_add((106).minutes())
        .unwrap()
        .checked_add((40).seconds())
        .unwrap()
}
