pub trait IntoFloat {
    fn as_f64(&self) -> f64;
}

// impl TypedFloat for f64 {
//     fn as_f64(&self) -> f64 {
//         *self
//     }
// }

// impl TypedFloat for f32 {
//     fn as_f64(&self) -> f64 {
//         (*self).into()
//     }
// }
