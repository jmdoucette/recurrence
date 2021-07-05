pub struct SolutionFunction {
    coefficients: Vec<f64>,
    bases: Vec<f64>,
}

impl SolutionFunction {
    pub fn new(coefficients: Vec<f64>, bases: Vec<f64>) {
        SolutionFunction {
            coefficients,
            bases,
        }
    }
}