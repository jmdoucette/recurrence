mod polynomial;
mod recurrence_relation;

fn main() {
    let p = polynomial::Polynomial::new(vec![1.0,-1.0,-1.0]);
    println!("{}, degree: {}, roots: {:?}", p, p.degree(), p.roots());
}