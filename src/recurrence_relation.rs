use crate::polynomial::Polynomial;
use crate::recurrence_solution::RecurrenceSolution;
use nalgebra::DMatrix;
use std::cmp::min;

pub struct RecurrenceRelation {
    base_cases: Vec<f64>,
    recurrence_coefficients: Vec<f64>,
}

impl RecurrenceRelation {
    /// creates a new recurrence relation with the specified recurrence and base cases
    /// for example the recurrence f_n = 3f_{n-1} + 5f_{n-2} and base cases f_0 = 0 and f_1 = 1
    /// base_cases = vec![0, 1]
    /// recurrence = vec![3, 5]
    pub fn new(base_cases: Vec<f64>, recurrence_coefficients: Vec<f64>) -> RecurrenceRelation {
        if base_cases.len() != recurrence_coefficients.len() {
            panic!("base case and recurrence must be same size")
        }

        RecurrenceRelation {
            base_cases,
            recurrence_coefficients,
        }
    }

    /// returns the degree of the recurrence
    pub fn degree(&self) -> usize {
        self.base_cases.len()
    }

    /// returns the characteristic polynomial of the recurrence
    pub fn characteristic_polynomial(&self) -> Polynomial {
        let mut coefficients = Vec::new();
        for coefficient in self.recurrence_coefficients.iter().rev() {
            coefficients.push(-1.0 * (*coefficient));
        }

        Polynomial::new(coefficients)
    }

    /// returns the polynomial which is an explicit solution to the recurrence relation
    pub fn solve(&self) -> RecurrenceSolution {
        let roots = self.characteristic_polynomial().roots();
        todo!();
        // change later
        /*
        let matrix =
            DMatrix::from_row_slice(2, 2, &[1.0, 1.0, 1.618033988749895, -0.6180339887498949]);
        let base_cases_vec = DMatrix::from_vec(self.degree(), 1, self.base_cases.clone());
        let alphas_matrix = matrix
            .lu()
            .solve(&base_cases_vec)
            .expect("cant solve given linear system");
        let alphas: Vec<f64> = alphas_matrix.iter().copied().collect();
        RecurrenceSolution::new(alphas, roots)
        */
    }

    /// returns the first n terms of the recurrence relation
    pub fn get_terms(&self, n: usize) -> Vec<f64> {
        let mut terms = Vec::new();

        for i in 0..min(n, self.degree()) {
            terms.push(self.base_cases[i])
        }

        for _ in self.degree()..n {
            let mut new_term = 0.0;
            for (term, coefficient) in terms.iter().rev().zip(self.recurrence_coefficients.iter()) {
                new_term += term * coefficient;
            }
            terms.push(new_term);
        }

        terms
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_characteristic_polynomial() {
        let recurrence_relation1 = RecurrenceRelation::new(vec![0.0, 1.0], vec![1.0, 1.0]);
        let characteristic_polynomial1 = Polynomial::new(vec![-1.0, -1.0]);
        assert_eq!(
            recurrence_relation1.characteristic_polynomial(),
            characteristic_polynomial1
        );

        let recurrence_relation2 =
            RecurrenceRelation::new(vec![1.0, -2.0, 3.0], vec![-6.0, -12.0, -8.0]);
        let characteristic_polynomial2 = Polynomial::new(vec![8.0, 12.0, 6.0]);
        assert_eq!(
            recurrence_relation2.characteristic_polynomial(),
            characteristic_polynomial2
        );
    }

    #[test]
    fn test_solve() {
        todo!();
    }

    #[test]
    fn test_get_terms() {
        let recurrence_relation1 = RecurrenceRelation::new(vec![0.0, 1.0], vec![1.0, 1.0]);
        let terms1 = vec![0.0, 1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0, 34.0];
        assert_eq!(recurrence_relation1.get_terms(10), terms1);

        let recurrence_relation2 =
            RecurrenceRelation::new(vec![1.0, -2.0, 3.0], vec![-6.0, -12.0, -8.0]);
        let terms2 = vec![
            1.0, -2.0, 3.0, -2.0, -8.0, 48.0, -176.0, 544.0, -1536.0, 4096.0,
        ];
        assert_eq!(recurrence_relation2.get_terms(10), terms2);
    }
}
