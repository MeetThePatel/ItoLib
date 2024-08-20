use crate::exercises::Exercise;
use itolib_time::DateTime;

#[derive(Debug, Copy, Clone)]
pub struct AmericanExercise {
    date: DateTime,
}

impl AmericanExercise {
    #[must_use]
    pub const fn new(date: DateTime) -> Self {
        Self { date }
    }
}

impl Exercise for AmericanExercise {
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
    use itolib_time::DateTime;

    use crate::exercises::{AmericanExercise, Exercise};

    #[test]
    fn test_american_exercise() {
        let now = DateTime::now();
        let x = AmericanExercise::new(now);
        assert_eq!(x.get_dates(), vec![now]);
        assert_eq!(x.get_last_date(), now);
    }
}
