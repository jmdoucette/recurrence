use crate::polynomial::Polynomial;
use crate::solution_functions::SolutionFunction;
use std::cmp::min;
use nalgebra::{DMatrix, };


pub struct RecurrenceRelation {
    base_cases: Vec<f64>,
    recurrence: Vec<f64>,
}

impl RecurrenceRelation {
    /// creates a new recurrence relation with the specified recurrence and base cases
    /// for example the recurrence f_n = 3f_{n-1} + 5f_{n-2} and base cases f_0 = 0 and f_1 = 1
    /// base_cases = vec![0, 1]
    /// recurrence = vec![3, 5]
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
    pub fn characteristic_polynomial(&self) -> Polynomial {
        let mut coefficients = Vec::new();
        for coefficient in self.recurrence.iter().rev() {
            coefficients.push(-1.0 * (*coefficient));
        }
        coefficients.push(1.0);

        Polynomial::new(coefficients)
    }

    /// returns the polynomial which is an explicit solution to the recurrence relation
    pub fn solve(&self) -> SolutionFunction {
        let roots = self.characteristic_polynomial().roots();
        let matrix = DMatrix::from_row_slice(2, 2, &[
            0.0, 1.0,
            1.0, 1.0
        ]);
        let base_cases_vec = DMatrix::from_vec(1, self.degree(), self.base_cases);
        let alphas = matrix.lu().solve(&base_cases_vec);

        SolutionFunction::new(roots, alphas)

    }

    /// returns the first n terms of the recurrence relation
    pub fn get_terms(&self, n: usize) -> Vec<f64> {
        let mut terms = Vec::new();

        for i in 0..min(n, self.degree()) {
            terms.push(self.base_cases[i])
        }

        for i in self.degree()..n {
            let mut new_term = 0.0;
            for (term, coefficient) in terms.iter().rev().zip(self.recurrence.iter()) {
                new_term += term * coefficient;
            }
            terms.push(new_term);
        }

        terms
    }

}