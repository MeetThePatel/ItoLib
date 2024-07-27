mod european_exercise;
pub use european_exercise::EuropeanExercise;

mod american_exercise;
pub use american_exercise::AmericanExercise;

use chrono::{DateTime, Utc};

pub trait Exercise {
    // This implementation is to allow for Bermudan options down the road.
    fn get_dates(&self) -> Vec<DateTime<Utc>>;

    fn get_last_date(&self) -> DateTime<Utc>;
}
