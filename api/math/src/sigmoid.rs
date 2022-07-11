pub fn sigmoid(x: f64) -> f64 {
    1. / (1. + (-x).exp())
}

pub fn logit(x: f64) -> f64 {
    (x / (1. - x)).ln()
}
