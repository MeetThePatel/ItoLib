use crate::{instruments::exercises::Exercise, time::DateTime};

#[derive(Debug, Copy, Clone)]
pub struct EuropeanExercise {
    date: DateTime,
}

impl EuropeanExercise {
    #[must_use]
    pub const fn new(date: DateTime) -> Self {
        Self { date }
    }
}

impl Exercise for EuropeanExercise {
    #[inline]
    #[must_use]
    fn get_dates(&self) -> Vec<DateTime> {
        vec![self.date]
    }

    #[inline]
    #[must_use]
    fn get_last_date(&self) -> DateTime {
        self.date
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        instruments::exercises::{EuropeanExercise, Exercise},
        time::DateTime,
    };

    #[test]
    fn test_european_exercise() {
        let now = DateTime::now();
        let x = EuropeanExercise::new(now);
        assert_eq!(x.get_dates(), vec![now]);
        assert_eq!(x.get_last_date(), now);
    }
}
