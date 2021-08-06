use recurrence::RecurrenceRelation;
use std::io::stdin;


fn main() {
    loop {
        println!("Enter recurrence relation");
        let mut recurrence_relation = String::new();
        stdin().read_line(&mut recurrence_relation).expect("unable to read");
        let recurrence_relation: RecurrenceRelation = match recurrence_relation.trim().parse() {
            Ok(recurrence_relation) => recurrence_relation,
            Err(_) => continue,
        };

        let recurrence_solution = recurrence_relation.solve();
        println!("solution is: {}", recurrence_solution);
    }
}

