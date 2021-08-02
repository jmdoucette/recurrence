#[macro_use]
extern crate rocket;

use recurrence::RecurrenceRelation;

#[get("/")]
fn index() -> String {
    let recurrence_solution = RecurrenceRelation::new(vec![0.0, 1.0], vec![1.0, 1.0]).solve();
    format!("solution to fibonacci: {:?}", recurrence_solution)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
