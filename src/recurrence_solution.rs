use std::fmt;
use crate::polynomial::Polynomial;

pub struct RecurrenceSolution {
    terms: Vec<(Polynomial, f64)>
}

impl RecurrenceSolution {
    pub fn new(terms: Vec<(Polynomial, f64)) -> RecurrenceSolution {
        RecurrenceSolution {
            terms
        }
    }

    pub fn evaluate(&self, n: u32) -> f64 {
        let mut res = 0.0;
        for (polynomial, root) in &self.terms {
            res += polynomial.evaluate(n) * root.powf(n as f64);
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
    fn test_get_terms() {
        todo!();
    }
}
