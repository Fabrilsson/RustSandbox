extern crate rayon;
extern crate stopwatch;

use rayon::prelude::*;
use stopwatch::Stopwatch;

fn main() {
    println!("Hello, world!");

    find_primes();
}

fn find_primes() -> i32 {

    let number = get_input().trim().parse::<i64>().unwrap();
    let outer_vector: Vec<_> = (2..number+1).collect();

    let mut sw = Stopwatch::start_new();

    sw.stop();
    sw.reset();
    sw.start();

    par_prime(&outer_vector);

    sw.stop();

    println!("Elapsed parallel: {:?}", sw.elapsed());

    println!();
    println!();

    sw.reset();
    sw.start();

    seq_prime(&outer_vector);

    sw.stop();

    println!("Elapsed sequential: {:?}", sw.elapsed());

    0
}

fn get_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).expect("Failed");
    buffer
}

fn par_prime(vector: &Vec<i64>) {
    let result: Vec<_> = vector.par_iter().map(|e| par_is_prime(e)).collect();

    for elem in result {
        match elem {
            Some(e) => print!("{:?}, ", e),
            _ => ()
        }
    }
}

fn seq_prime(vector: &Vec<i64>) {
    let result: Vec<i64> = seq_is_prime(vector);

    println!("Primes: {:?}", result);
}

fn par_is_prime(e: &i64) -> Option<&i64>{
    let inner_vector: Vec<_> = (2..(*e)).collect();

    let mut is_prime = true;

    for inner_elem in inner_vector {
        if *e % inner_elem == 0 {
            is_prime = false;
        }
    }

    if is_prime {
        return Some(e);
    }

    None
}

fn seq_is_prime(vector: &Vec<i64>) -> Vec<i64>{

    let mut primes: Vec<i64> = Vec::new();

    for outer_elem in vector {

        let inner_vector: Vec<_> = (2..(*outer_elem)).collect();

        let mut is_prime = true;

        for inner_elem in inner_vector {
            if outer_elem % inner_elem == 0 {
                is_prime = false;
            }
        }

        if is_prime {
            primes.push(*outer_elem);
        }
    }

    primes
}