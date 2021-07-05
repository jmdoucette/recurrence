#[allow(dead_code)]
const TOL: f64 = 0.000001;

#[allow(dead_code)]
pub fn within(x: f64, y: f64) -> bool {
    (x - y).abs() > TOL
}

#[allow(dead_code)]
pub fn assert_within(x: f64, y: f64) {
    if !within(x, y) {
        panic!(
            "assertion failed |left - right| < TOL\n left = {}\n right = {}\n",
            x, y
        );
    }
}
