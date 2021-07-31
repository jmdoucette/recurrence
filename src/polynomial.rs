use crate::utilities::*;
use nalgebra::DMatrix;
use std::fmt;

#[derive(Debug, PartialEq)]
pub struct Polynomial {
    coefficients: Vec<f64>,
}

impl Polynomial {
    // creates a new polynomial from the given coefficients
    // starts with the coefficient of the constant terms
    // leading terms has no coefficient but is always considered to be 1
    pub fn new(coefficients: Vec<f64>) -> Polynomial {
        // remove leading zero terms
        Polynomial { coefficients }
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len()
    }

    // returns the companion matrix of the polynomial
    pub fn companion_matrix(&self) -> DMatrix<f64> {
        let mut elements = Vec::new();
        for (i, coefficient) in self.coefficients.iter().enumerate() {
            for j in 0..self.degree() - 1 {
                if i == j + 1 {
                    elements.push(1.0);
                } else {
                    elements.push(0.0);
                }
            }
            elements.push(-1.0 * coefficient);
        }
        DMatrix::from_row_slice(self.degree(), self.degree(), &elements)
    }

    // returns a vector of the roots of a polynomial
    pub fn roots(&self) -> Vec<(f64, u32)> {
        let companion = self.companion_matrix();
        // look into increasing number of iterations
        let schur = companion
            .try_schur(0.000000000000000001, 0)
            .unwrap()
            .unpack()
            .1;

        let mut eigenvalues = Vec::new();
        // todo: what to on fail?

        for (i, column) in schur.column_iter().enumerate() {
            let mut found = false;
            let new_eigenvalue = column[i];
            for (eigenvalue, count) in &mut eigenvalues {
                if within(new_eigenvalue, *eigenvalue) {
                    *count += 1;
                    found = true;
                    break;
                }
            }
            if !found {
                eigenvalues.push((new_eigenvalue, 1));
            }
        }
        eigenvalues
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
    use super::*;

    #[test]
    fn test_companion_matrix() {
        let polynomial1 = Polynomial::new(vec![-1.0, -1.0]);
        let expected_companion_matrix1 = DMatrix::from_row_slice(2, 2, &[0.0, 1.0, 1.0, 1.0]);
        assert_eq!(polynomial1.companion_matrix(), expected_companion_matrix1);

        let polynomial2 = Polynomial::new(vec![8.0, 12.0, 6.0]);
        let expected_companion_matrix2 =
            DMatrix::from_row_slice(3, 3, &[0.0, 0.0, -8.0, 1.0, 0.0, -12.0, 0.0, 1.0, -6.0]);
        assert_eq!(polynomial2.companion_matrix(), expected_companion_matrix2);

        let polynomial3 = Polynomial::new(vec![3.24, 6.52, -5.1, -0.12, 1.0]);
        let expected_companion_matrix3 = DMatrix::from_row_slice(
            5,
            5,
            &[
                0.0, 0.0, 0.0, 0.0, -3.24, 1.0, 0.0, 0.0, 0.0, -6.52, 0.0, 1.0, 0.0, 0.0, 5.1, 0.0,
                0.0, 1.0, 0.0, 0.12, 0.0, 0.0, 0.0, 1.0, -1.0,
            ],
        );
        assert_eq!(polynomial3.companion_matrix(), expected_companion_matrix3);
    }

    #[test]
    fn test_roots() {
        let polynomial1 = Polynomial::new(vec![-1.0, -1.0]);
        let expected_roots1 = vec![
            (1.618033988749894848204586834, 1),
            (-0.618033988749894848204586834, 1),
        ];
        assert!(float_counts_equal(polynomial1.roots(), expected_roots1));

        let polynomial2 = Polynomial::new(vec![8.0, 12.0, 6.0]);
        let expected_roots2 = vec![(-2.0, 3)];
        assert!(float_counts_equal(polynomial2.roots(), expected_roots2));
    }
}
