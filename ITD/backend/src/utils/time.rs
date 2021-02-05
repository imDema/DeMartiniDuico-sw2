use chrono::prelude::*;

pub fn minute_diff(from: NaiveDateTime, to: NaiveDateTime) -> f32 {
    let duration = to - from;
    let millis = duration.num_milliseconds();
    millis as f32 / 60000.
}

pub fn combine_expected_measured(est_visit: f32, visit: f32) -> f32 {
    est_visit * 0.35 + visit * 0.65 + 2.0
}