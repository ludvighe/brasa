use chrono::{Datelike, Local, NaiveDate, NaiveDateTime};

pub struct TimeToChristmas {
    christmas_time: NaiveDateTime,
    current_time: NaiveDateTime,
}

impl TimeToChristmas {
    pub fn new() -> Self {
        let current_year = Local::now().year();
        let christmas_time = NaiveDate::from_ymd_opt(current_year, 12, 24)
            .unwrap()
            .and_hms_opt(0, 0, 0)
            .unwrap();

        Self {
            christmas_time,
            current_time: Local::now().naive_local(),
        }
    }

    pub fn is_christmas(&self) -> bool {
        let today = self.current_time.date();
        let christmas = self.christmas_time.date();
        today == christmas
    }

    pub fn time_until_christmas(&self) -> (i64, i64, i64) {
        if self.is_christmas() {
            (0, 0, 0)
        } else {
            let duration = self.christmas_time - self.current_time;
            let days = duration.num_days();
            let hours = duration.num_hours() % 24;
            let minutes = duration.num_minutes() % 60;
            (days, hours, minutes)
        }
    }
}
