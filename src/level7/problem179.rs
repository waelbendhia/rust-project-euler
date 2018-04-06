use problem;

pub fn problem() -> problem::Problem {
    return problem::Problem {
        ind: 179,
        name: String::from("Consecutive positive divisors"),
        solution: solution,
    };
}

fn solution() -> i64 {
    return count_consecutive(all_num_divisors_under(10i64.pow(7)));
}

fn count_consecutive(divs: Vec<i64>) -> i64 {
    let mut total: i64 = 0;
    for i in 0..divs.len() - 2 {
        if divs[i] == divs[i + 1] {
            total += 1;
        }
    }
    return total;
}

fn all_num_divisors_under(b: i64) -> Vec<i64> {
    let mut divs = vec![0; (b + 1) as usize];
    for i in 1..b {
        let mut j = 1;
        while j < b {
            divs[j as usize] = divs[j as usize] + 1;
            j += i;
        }
    }
    return divs;
}

// This just checks all factors and increments an array
// the above function turned out to be about 30% faster.
// fn all_num_divisors_under(b: i64) -> Vec<i64> {
//     let mut divs = vec![0; (b + 1) as usize];
//     for i in 1..b {
//         for j in 1..b {
//             let ind = i * j;
//             if ind > b {
//                 break;
//             }
//             divs[ind as usize] = divs[ind as usize] + 1
//         }
//     }
//     return divs;
// }
