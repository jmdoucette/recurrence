use crate::polynomial::Polynomial;

pub struct RecurrenceRelation {
    base_cases: Vec<f64>,
    recurrence: Vec<f64>,
}

impl RecurrenceRelation {
    /// creates a new recurrence relation with the specified recurrence and base cases
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
        todo!();
    }

    /// returns the polynomial which is an explicit solution to the recurrence relation
    pub fn solve(&self) -> Polynomial {
        todo!();
    }

    /// returns the first n terms of the recurrence relation
    pub fn get_terms(&self, n: u32) -> Vec<f64> {
        todo!();
    }

}