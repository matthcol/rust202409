pub fn inverted_square(x: &f64) -> f64 {
    1.0 / x * x
}

pub fn in_range(value: f64, min: f64, max: f64) -> bool {
    // let res = match value {
    //     min..max => true,
    //     _ => false
    // };

    // let res = if (value >= min) && (value < max) { true } else {false};
    // res;

    // (value >= min) && (value < max);

    (min..max).contains(&value)
}