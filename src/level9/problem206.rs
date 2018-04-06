use problem;

pub fn problem() -> problem::Problem {
    return problem::Problem {
        ind: 206,
        name: String::from("Concealed Square"),
        solution: solution,
    };
}

fn solution() -> i64 {
    let opt = find_square_for_shape(String::from("1_2_3_4_5_6_7_8_9_0"));
    return int_sqrt(opt.unwrap_or(0)) as i64;
}

fn find_square_for_shape(shape: String) -> Option<u64> {
    match range_from_shape(&shape) {
        Some((lo, hi)) => {
            let mut i = int_sqrt(lo);
            let mut j = i * i;
            while i <= int_sqrt(hi) {
                if validate_shape(&shape, j) {
                    return Some(j);
                }
                i += 10;
                j = i * i;
            }
            return None;
        }
        None => return None,
    }
}

fn range_from_shape(shape: &String) -> Option<(u64, u64)> {
    match shape.replace("_", "0").parse::<u64>() {
        Ok(lo) => match shape.replace("_", "9").parse::<u64>() {
            Ok(hi) => return Some((lo, hi)),
            Err(_) => return None,
        },
        Err(_) => {
            return None;
        }
    }
}

fn validate_shape(shape: &String, num: u64) -> bool {
    let num_str = num.to_string();
    if shape.len() != num_str.len() {
        return false;
    }
    let mut shape_iter = shape.chars();
    let mut num_iter = num_str.chars();
    loop {
        match shape_iter.next() {
            Some(s) => match num_iter.next() {
                Some(n) => {
                    if s != '_' && s != n {
                        return false;
                    }
                }
                None => break,
            },
            None => break,
        }
    }
    return true;
}

fn int_sqrt(n: u64) -> u64 {
    return (n as f64).sqrt() as u64;
}
