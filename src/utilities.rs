use std::collections::HashSet;

#[allow(dead_code)]
// may cause some false positives
const TOL: f64 = 0.001;

// NOT equivalece relations as is not transitive
#[allow(dead_code)]
pub fn within(x: f64, y: f64) -> bool {
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
pub fn float_counts_within(fc1: Vec<(f64, usize)>, fc2: Vec<(f64, usize)>) -> bool {
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

#[allow(dead_code)]
pub fn vec_within(v1: Vec<f64>, v2: Vec<f64>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }
    for (num1, num2) in v1.iter().zip(v2.iter()) {
        if !within(*num1, *num2) {
            return false
        }
    }
    true
}



