use crate::polynomial::Polynomial;
use std::fmt;

#[derive(Debug, Clone)]
pub struct RecurrenceSolution {
    terms: Vec<(Polynomial, f64)>,
}

impl RecurrenceSolution {
    pub fn new(terms: Vec<(Polynomial, f64)>) -> RecurrenceSolution {
        RecurrenceSolution { terms }
    }

    fn evaluate(&self, n: u32) -> f64 {
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

        for (i, (polynomial, root)) in self.terms.iter().enumerate() {
            if i == self.terms.len()-1 {
                res.push_str(&format!("({}) * {:.3}^n", polynomial, root));
            } else {
                res.push_str(&format!("({}) * {:.3}^n + ", polynomial, root));
            }
        }
        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::*;

    #[test]
    fn test_get_terms() {
        let recurrence_solution1 = RecurrenceSolution::new(vec![
            (Polynomial::new(vec![0.4472135955]), 1.618_033_988_749_895),
            (
                Polynomial::new(vec![-0.4472135955]),
                -0.618_033_988_749_894_9,
            ),
        ]);
        let terms1 = vec![0.0, 1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0];
        assert!(vec_within(recurrence_solution1.get_terms(10), terms1));

        // todo: add testcase with multiplicity greater than 1
    }
}
