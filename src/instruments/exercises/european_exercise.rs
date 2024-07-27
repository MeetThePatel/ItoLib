use crate::instruments::Exercise;

use chrono::{DateTime, Utc};

#[derive(Debug, Copy, Clone)]
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
    #[inline]
    #[must_use]
    fn get_dates(&self) -> Vec<DateTime<Utc>> {
        vec![self.date]
    }

    #[inline]
    #[must_use]
    fn get_last_date(&self) -> DateTime<Utc> {
        self.date
    }
}

#[cfg(test)]
mod tests {
    use crate::instruments::{EuropeanExercise, Exercise};

    use chrono::Utc;

    #[test]
    fn test_european_exercise() {
        let now = Utc::now();
        let x = EuropeanExercise::new(now);
        assert_eq!(x.get_dates(), vec![now]);
        assert_eq!(x.get_last_date(), now);
    }
}
