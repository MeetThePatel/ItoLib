#[macro_export]
macro_rules! any_true {
    () => {
        None
    };
    ($single:expr) => {
        if $single {
            Some(0)
        } else {
            None
        }
    };
    ($first:expr, $($rest:expr),*) => {
        if $first {
            Some(0)
        } else {
            match any_true!($($rest),*) {
                Some(index) => Some(index + 1),
                None => None,
            }
        }
    };
}
