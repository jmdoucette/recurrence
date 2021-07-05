mod polynomial;
use polynomial::Polynomial;

mod recurrence_relation;
use recurrence_relation::RecurrenceRelation;

fn main() {
    let p = Polynomial::new(vec![-1.0,-1.0,1.0]);
    println!("{}, degree: {}, roots: {:?}", p, p.degree(), p.roots());

    let base_cases = vec![1.0,6.0];
    let coefficients = vec![6.0,-9.0];

    let r = RecurrenceRelation::new(base_cases, coefficients);
    r.solve();
    println!("terms: {:?}, characteristic polynomial: {}", r.get_terms(10), r.characteristic_polynomial());
}