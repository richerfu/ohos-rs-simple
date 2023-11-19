mod value;

use back_macro::napi;

#[napi]
pub fn add_1(a: f64, b: f64) -> f64 {
    a + b
}
