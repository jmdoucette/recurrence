use std::fmt;

pub struct RecurrenceSolution {
    coefficients: Vec<f64>,
    bases: Vec<f64>,
}

impl RecurrenceSolution {
    pub fn new(coefficients: Vec<f64>, bases: Vec<f64>) -> RecurrenceSolution {
        if coefficients.len() != bases.len() {
            panic!("must have same length");
        }

        RecurrenceSolution {
            coefficients,
            bases,
        }
    }

    pub fn evaluate(&self, n: u32) -> f64 {
        let mut res = 0.0;
        for (base, coefficient) in self.bases.iter().zip(self.coefficients.iter()) {
            res += coefficient * base.powf(n as f64);
        }
        
        res
    }

    pub fn get_terms(&self, n: u32) -> Vec<f64> {
        let mut res = Vec::new();
        for i in 0..n {
            res.push(self.evaluate(i));
        }

        res
    }

}

impl fmt::Display for RecurrenceSolution {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        for (base, coefficient) in self.bases.iter().zip(self.coefficients.iter()) {
            res.push_str(&format!("{} * {}^n ", coefficient, base));
        }
        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    //use crate::test_utils::*;

    #[test]
    fn test_evaluate() {
        todo!();
    }

    #[test]
    fn test_get_terms () {
        todo!();
    }
}
