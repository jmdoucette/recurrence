use crate::polynomial::Polynomial;
use crate::recurrence_solution::RecurrenceSolution;
use nalgebra::DMatrix;
use std::cmp::{max, min};
use std::str::FromStr;

#[derive(PartialEq, Debug)]
pub struct RecurrenceRelation {
    base_cases: Vec<f64>,
    recurrence_coefficients: Vec<f64>,
}

impl RecurrenceRelation {
    /// creates a new recurrence relation with the specified recurrence and base cases
    /// for example the recurrence f(n) = 3f(n-1) + 5f(n-2) and base cases f(0) = 0 and f(1) = 1
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
        coefficients.push(1.0);

        Polynomial::new(coefficients)
    }

    /// returns the polynomial which is an explicit solution to the recurrence relation
    pub fn solve(&self) -> RecurrenceSolution {
        let roots = self.characteristic_polynomial().roots();
        let mut elements = Vec::new();
        for n in 0..self.degree() {
            for (root, count) in &roots {
                for i in 0..*count {
                    elements.push(root.powf(n as f64) * (n as f64).powf(i as f64));
                }
            }
        }

        let matrix = DMatrix::from_row_slice(self.degree(), self.degree(), &elements);
        let base_cases_vec = DMatrix::from_row_slice(self.degree(), 1, &self.base_cases);

        let alphas_matrix = matrix
            .lu()
            .solve(&base_cases_vec)
            .expect("cant solve given linear system");
        let alphas: Vec<f64> = alphas_matrix.iter().copied().collect();

        let mut terms = Vec::new();
        let mut index = 0;
        for (root, count) in &roots {
            let mut polynomial_coefficients = Vec::new();
            for _ in 0..*count {
                polynomial_coefficients.push(alphas[index]);
                index += 1;
            }
            let polynomial = Polynomial::new(polynomial_coefficients);
            terms.push((polynomial, *root));
        }
        RecurrenceSolution::new(terms)
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

#[derive(Debug)]
pub enum ParseRecurrenceError {
    NoRecurrence,
    MultipleRecurrence,
    TermMatchesNothing,
    MultipleBaseCase,
    NoBaseCase,
    MissingEquals,
    ParseFloatError,
    ParseIntError,
    BaseCaseError,
    RecurrenceError,
}

impl From<std::num::ParseFloatError> for ParseRecurrenceError {
    fn from(_: std::num::ParseFloatError) -> Self {
        ParseRecurrenceError::ParseFloatError
    }
}

impl From<std::num::ParseIntError> for ParseRecurrenceError {
    fn from(_: std::num::ParseIntError) -> Self {
        ParseRecurrenceError::ParseIntError
    }
}

fn parse_base_case(s: &str) -> Result<(f64, usize), ParseRecurrenceError> {
    let mut parts = s.split('=');
    //todo: turn panic to error
    let left = parts.next().ok_or(ParseRecurrenceError::BaseCaseError)?;
    let right = parts.next().ok_or(ParseRecurrenceError::BaseCaseError)?;
    if parts.next().is_some() {
        return Err(ParseRecurrenceError::RecurrenceError);
    }
    let lparen = left.find('{').ok_or(ParseRecurrenceError::BaseCaseError)?;
    let rparen = left.find('}').ok_or(ParseRecurrenceError::BaseCaseError)?;
    let index: usize = left[lparen + 1..rparen].trim().parse()?;

    let val: f64 = right.trim().parse()?;
    Ok((val, index))
}

fn parse_recurrence(s: &str) -> Result<Vec<f64>, ParseRecurrenceError> {
    let mut parts = s.split('=');
    let _left = parts.next().ok_or(ParseRecurrenceError::RecurrenceError)?;
    let right = parts.next().ok_or(ParseRecurrenceError::RecurrenceError)?;
    if parts.next().is_some() {
        return Err(ParseRecurrenceError::RecurrenceError);
    }

    let parts = right.split('+');
    let mut degree = 0;
    let mut pairs: Vec<(f64, usize)> = Vec::new();
    for part in parts {
        let part = part.trim();
        let lparen = part
            .find('{')
            .ok_or(ParseRecurrenceError::RecurrenceError)?;
        let rparen = part
            .find('}')
            .ok_or(ParseRecurrenceError::RecurrenceError)?;
        let minus = part
            .find('-')
            .ok_or(ParseRecurrenceError::RecurrenceError)?;

        let index: usize = part[minus + 1..rparen].trim().parse()?;
        degree = max(degree, index);
        let coefficient;
        if lparen == 1 {
            coefficient = 1.0;
        } else {
            coefficient = part[..lparen - 1].trim().parse()?;
        }
        pairs.push((coefficient, index));
    }

    let mut res = vec![0.0; degree];
    for (coefficient, index) in pairs {
        res[index - 1] = coefficient;
    }
    Ok(res)
}

impl FromStr for RecurrenceRelation {
    type Err = ParseRecurrenceError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut recurrence = None;
        let mut base_case_pairs = Vec::new();
        let equations = s.split(',').map(|x| x.trim());

        for equation in equations {
            if let Ok(parsed_base_case) = parse_base_case(equation) {
                base_case_pairs.push(parsed_base_case);
            } else if let Ok(parsed_recurrence) = parse_recurrence(equation) {
                if recurrence.is_some() {
                    return Err(ParseRecurrenceError::MultipleRecurrence);
                }
                recurrence = Some(parsed_recurrence);
            } else {
                return Err(ParseRecurrenceError::TermMatchesNothing);
            }
        }

        let recurrence = recurrence.ok_or(ParseRecurrenceError::NoRecurrence)?;

        // should look into what happens with trailing 0s
        let degree = recurrence.len();
        let mut base_cases = vec![None; degree];
        for (num, index) in base_case_pairs {
            // may panic here, should swap to error
            if base_cases[index].is_some() {
                return Err(ParseRecurrenceError::MultipleBaseCase);
            } else {
                base_cases[index] = Some(num);
            }
        }
        let base_cases = base_cases
            .iter()
            .map(|x| x.ok_or(ParseRecurrenceError::NoBaseCase))
            .collect::<Result<Vec<f64>, ParseRecurrenceError>>()?;
        Ok(RecurrenceRelation::new(base_cases, recurrence))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utilities::*;

    #[test]
    fn test_characteristic_polynomial() {
        let recurrence_relation1 = RecurrenceRelation::new(vec![0.0, 1.0], vec![1.0, 1.0]);
        let characteristic_polynomial1 = Polynomial::new(vec![-1.0, -1.0, 1.0]);
        assert_eq!(
            recurrence_relation1.characteristic_polynomial(),
            characteristic_polynomial1
        );

        let recurrence_relation2 =
            RecurrenceRelation::new(vec![1.0, -2.0, 3.0], vec![-6.0, -12.0, -8.0]);
        let characteristic_polynomial2 = Polynomial::new(vec![8.0, 12.0, 6.0, 1.0]);
        assert_eq!(
            recurrence_relation2.characteristic_polynomial(),
            characteristic_polynomial2
        );
    }

    #[test]
    fn test_solve() {
        let recurrence_relation1 = RecurrenceRelation::new(vec![0.0, 1.0], vec![1.0, 1.0]);
        assert!(vec_within(
            recurrence_relation1.solve().get_terms(10),
            recurrence_relation1.get_terms(10)
        ));

        let recurrence_relation2 =
            RecurrenceRelation::new(vec![1.0, -2.0, 3.0], vec![-6.0, -12.0, -8.0]);
        assert!(vec_within(
            recurrence_relation2.solve().get_terms(5),
            recurrence_relation2.get_terms(5)
        ));
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

    #[test]
    fn test_parse_base_case() {
        assert_eq!(parse_base_case("f(0) = 1.0").unwrap(), (1.0, 0));
        assert_eq!(parse_base_case("f(0) = 1").unwrap(), (1.0, 0));
        assert_eq!(parse_base_case("f(2) = 3.24").unwrap(), (3.24, 2));
        assert_eq!(parse_base_case("a(2) = 3.24").unwrap(), (3.24, 2));
    }

    #[test]
    fn test_parse_recurrence() {
        assert_eq!(parse_recurrence("f(n) = 1.0f(n-1)").unwrap(), vec![1.0]);
        assert_eq!(parse_recurrence("f(n) = 1f(n-1)").unwrap(), vec![1.0]);
        assert_eq!(parse_recurrence("f(n) = f(n-1)").unwrap(), vec![1.0]);
        assert_eq!(parse_recurrence("f(n) = 3.24f(n-1)").unwrap(), vec![3.24]);

        assert_eq!(
            parse_recurrence("f(n) = f(n-1) + f(n-2)").unwrap(),
            vec![1.0, 1.0]
        );
        assert_eq!(
            parse_recurrence("f(n) = 3f(n-1) + 5f(n-3)").unwrap(),
            vec![3.0, 0.0, 5.0]
        );
    }

    #[test]
    fn test_parse_recurrence_relation() {
        let relation: RecurrenceRelation = "f(n) = f(n-1) + f(n-2), f(0) = 0, f(1) = 1"
            .parse()
            .unwrap();
        assert_eq!(
            relation,
            RecurrenceRelation::new(vec![0.0, 1.0], vec![1.0, 1.0])
        );
    }
}
