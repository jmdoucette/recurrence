mod polynomial;
use polynomial::Polynomial;

mod recurrence_relation;
use recurrence_relation::RecurrenceRelation;

fn main() {
    let p = Polynomial::new(vec![-1.0,-1.0,1.0]);
    println!("{}, degree: {}, roots: {:?}", p, p.degree(), p.roots());

    let base_cases = vec![0.0,1.0];
    let coefficients = vec![1.0,1.0];

    let r = RecurrenceRelation::new(base_cases, coefficients);
    println!("terms: {:?}, characteristic polynomial: {}, solution: {}", r.get_terms(10), r.characteristic_polynomial(), r.solve());
}