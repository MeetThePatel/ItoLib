pub mod european_exercise;
pub use european_exercise::*;

use chrono::{DateTime, Utc};

pub trait Exercise {
    type DateContainer;

    // This implementation is to allow for Bermudan options down the road.
    fn get_dates(&self) -> &Self::DateContainer;

    fn get_last_date(&self) -> DateTime<Utc>;
}
