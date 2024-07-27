use crate::instruments::Exercise;

use chrono::{DateTime, Utc};

pub struct EuropeanExercise {
    date: DateTime<Utc>,
}

impl EuropeanExercise {
    #[must_use]
    pub const fn new(date: DateTime<Utc>) -> Self {
        Self { date }
    }
}

impl Exercise for EuropeanExercise {
    type DateContainer = DateTime<Utc>;

    #[inline]
    #[must_use]
    fn get_dates(&self) -> &Self::DateContainer {
        &self.date
    }

    #[inline]
    #[must_use]
    fn get_last_date(&self) -> Self::DateContainer {
        self.date
    }
}
