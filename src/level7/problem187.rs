use problem;
extern crate primal;

pub fn problem() -> problem::Problem {
    return problem::Problem {
        ind: 187,
        name: String::from("Semiprimes"),
        solution: solution,
    };
}

// This algorithm runs in 16 seconds on my machine, which is okay, but there
// are probably some further optimizations I can make.
fn solution() -> i64 {
    let facts = all_prime_factorizations_under(10i64.pow(7));
    let mut count = 0;
    for f in facts.iter() {
        if *f == 2 {
            count += 1;
        }
    }
    return count;
}

fn all_prime_factorizations_under(b: i64) -> Vec<i64> {
    let mut facts = vec![0; b as usize];
    let primes = primal::Primes::all().take_while(|p| *p < b as usize);
    for p in primes {
        let mut i = p;
        while i < b as usize {
            facts[i as usize] += rough_log(p, i) as i64;
            i += p;
        }
    }
    return facts;
}

fn rough_log(b: usize, n: usize) -> usize {
    let mut j = n;
    let mut log = 0;
    while j % b == 0 {
        log += 1;
        j /= b;
    }
    return log;
}
