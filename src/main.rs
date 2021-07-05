mod polynomial;
use polynomial::Polynomial;

mod recurrence_relation;
use recurrence_relation::RecurrenceRelation;

mod recurrence_solution;
//use recurrence_solution::RecurrenceSolution;

mod test_utils;

fn main() {
    println!("testing");
    let p = Polynomial::new(vec![-1.0,-1.0,1.0]);
    println!("{}, degree: {}, roots: {:?}", p, p.degree(), p.roots());

    let base_cases = vec![0.0,1.0];
    let coefficients = vec![1.0,1.0];

    let r = RecurrenceRelation::new(base_cases, coefficients);
    println!("terms: {:?}, terms from solution: {:?}", r.get_terms(10), r.solve().get_terms(10));
}
