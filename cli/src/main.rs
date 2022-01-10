use recurrence::RecurrenceRelation;
use std::io::stdin;

fn main() {
    loop {
        println!("Enter recurrence relation:");
        let mut recurrence_relation_string = String::new();
        stdin()
            .read_line(&mut recurrence_relation_string)
            .expect("unable to read");
        let recurrence_relation: RecurrenceRelation =
            match recurrence_relation_string.trim().parse() {
                Ok(recurrence_relation) => recurrence_relation,
                Err(_) => continue,
            };
        let recurrence_solution = recurrence_relation.solve();
        println!("The solution to this recurrence relation is: {}", recurrence_solution);
        println!("The first 10 terms of this recurrence relation are: {:?}", recurrence_relation.get_terms(10))
    }
}
