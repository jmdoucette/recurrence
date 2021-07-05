use nalgebra::DMatrix;
use std::fmt;

pub struct Polynomial {
    coefficients: Vec<f64>,
}

impl Polynomial {
    // creates a new polynomial from the given coefficients
    // starts with the coefficient of the smallest exponent
    pub fn new(coefficients: Vec<f64>) -> Polynomial {
        Polynomial { coefficients }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len()
    }

    // returns the companion matrix of the polynomial
    pub fn companion_matrix(&self) -> DMatrix<f64> {
        DMatrix::from_row_slice(2, 2, &[0.0, 1.0, 1.0, 1.0])
    }

    // returns a vector of the roots of a polynomial
    pub fn roots(&self) -> Vec<f64> {
        let companion = self.companion_matrix();
        let eigs = companion.schur().unpack().1;

        let mut res = Vec::new();
        for (i, column) in eigs.column_iter().enumerate() {
            res.push(column[i]);
        }

        res
    }
}

impl fmt::Display for Polynomial {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut res = String::new();
        for (pow, coefficient) in self.coefficients.iter().enumerate().rev() {
            res.push_str(&format!("{}x^{} ", coefficient, pow));
        }
        write!(f, "{}", res)
    }
}

#[cfg(test)]
mod tests {
    //use super::*;
    //use crate::test_utils::*;

    #[test]
    fn test_companion_matrix() {
        todo!();
    }

    #[test]
    fn test_roots() {
        todo!();
    }
}
