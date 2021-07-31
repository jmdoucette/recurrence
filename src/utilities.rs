use std::collections::HashSet;

#[allow(dead_code)]
const TOL: f64 = 0.0001;

// NOT equivalece relations as is not transitive
#[allow(dead_code)]
pub fn within(x: f64, y: f64) -> bool {
    println!("{} {}", (x - y).abs(), TOL);
    (x - y).abs() < TOL
}

#[allow(dead_code)]
pub fn assert_within(x: f64, y: f64) {
    if !within(x, y) {
        panic!(
            "assertion failed |left - right| < TOL\n left = {}\n right = {}\n",
            x, y
        );
    }
}

#[allow(dead_code)]
pub fn float_counts_equal(fc1: Vec<(f64, u32)>, fc2: Vec<(f64, u32)>) -> bool {
    let mut used = HashSet::new();

    for (float1, count1) in &fc1 {
        for (i, (float2, count2)) in fc2.iter().enumerate() {
            if within(*float1, *float2) {
                if *count1 != *count2 || used.contains(&i) {
                    return false;
                } else {
                    used.insert(i);
                    break;
                }
            }
        }
    }
    fc1.len() == fc2.len() && fc1.len() == used.len()
}
