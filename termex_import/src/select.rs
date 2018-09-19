use chrono::{DateTime, Duration, Utc};

pub struct PastDays(pub u64, pub u64);

impl PastDays {
    pub fn past(days: u32) -> Self {
        let utcnow: DateTime<Utc> = Utc::now();
        let duration: Duration = Duration::days(days as i64);
        let past_time = utcnow - duration;
        PastDays(utcnow.timestamp() as u64, past_time.timestamp() as u64)
    }
}
