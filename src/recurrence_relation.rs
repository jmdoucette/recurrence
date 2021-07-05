use crate::polynomial::Polynomial;
use std::cmp::min;

pub struct RecurrenceRelation {
    base_cases: Vec<f64>,
    recurrence: Vec<f64>,
}

impl RecurrenceRelation {
    /// creates a new recurrence relation with the specified recurrence and base cases
    /// for example the fibonacci sequence which has recurrence f_n = f_{n-1} + f_{n-2} and base cases f_0 = 0 and f_1 = 1
    /// base_cases = vec![0, 1]
    /// recurrence = vec![1, 1]
    pub fn new(base_cases: Vec<f64>, recurrence: Vec<f64>) -> RecurrenceRelation {
        if base_cases.len() != recurrence.len() {
            panic!("base case and recurrence must be same size")
        }

        RecurrenceRelation {
            base_cases,
            recurrence,
        }
    }

    /// returns the degree of the recurrence
    pub fn degree(&self) -> usize {
        return self.base_cases.len();
    }

    /// returns the characteristic polynomial of the recurrence
    fn characteristic_polynomial(&self) -> Polynomial {
        let mut coefficients = vec![1.0];
        for coefficient in &self.recurrence {
            coefficients.push(*coefficient);
        }
        
        Polynomial::new(coefficients)
    }

    /// returns the polynomial which is an explicit solution to the recurrence relation
    pub fn solve(&self) -> Polynomial {
        todo!();
    }

    /// returns the first n terms of the recurrence relation
    pub fn get_terms(&self, n: usize) -> Vec<f64> {
        let mut terms = Vec::new();

        for i in 0..min(n, self.degree()) {
            terms.push(self.base_cases[i])
        }

        for i in self.degree()..n {
            let mut term = 0.0;
            for (base_case, coefficient) in self.base_cases.iter().zip(self.recurrence.iter()) {
                term += base_case * coefficient;
            }
            terms.push(term);
        }

        terms
    }

}