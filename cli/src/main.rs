use recurrence::RecurrenceRelation;
use std::io::stdin;


fn main() {
    loop {
        println!("Enter recurrence relation");
        let mut recurrence_relation_string = String::new();
        stdin().read_line(&mut recurrence_relation_string).expect("unable to read");
        let recurrence_relation: RecurrenceRelation = match recurrence_relation_string.trim().parse() {
            Ok(recurrence_relation) => recurrence_relation,
            Err(e) => {
                continue
            }
        };
        let recurrence_solution = recurrence_relation.solve();
        println!("solution is: {}", recurrence_solution);
    }
}

