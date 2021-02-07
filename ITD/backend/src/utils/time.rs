use chrono::prelude::*;

/// Get length of interval from `from` to `to` in `f32` minutes
pub fn minute_diff(from: NaiveDateTime, to: NaiveDateTime) -> f32 {
    let duration = to - from;
    let millis = duration.num_milliseconds();
    millis as f32 / 60000.
}

/// Combine moving averages for estimated and measured visit length
pub fn combine_expected_measured(est_visit: f32, visit: f32) -> f32 {
    est_visit * 0.35 + visit * 0.65 + 2.0
}