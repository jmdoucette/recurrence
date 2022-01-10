use recurrence::RecurrenceRelation;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Cli {
    /// The string representing the recurrence relation  to look for
    recurrence_relation_string: String,
}

fn main() {
    let args = Cli::from_args();
    let recurrence_relation: RecurrenceRelation =
        match args.recurrence_relation_string.trim().parse() {
            Ok(recurrence_relation) => recurrence_relation,
            Err(_) => {
                println!("invalid recurrence relation");
                return;
            }
        };

    let recurrence_solution = recurrence_relation.solve();
    println!(
        "The solution to this recurrence relation is: {}",
        recurrence_solution
    );
    println!(
        "The first 10 terms of this recurrence relation are: {:?}",
        recurrence_relation.get_terms(10)
    );
}
