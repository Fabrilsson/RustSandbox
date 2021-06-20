use std::time::Instant;
use rayon::prelude::*;

fn main() {
    println!("Hello, world!");

    // let array: [i32; 3] = [1, 2, 3]; 

    // let result = sum_of_squares(&array);

    // println!("teste = {}", result);

    let mut start = Instant::now();
    
    fibonacci_split_recursive();

    let mut dur = Instant::now() - start;
    let mut nanos = u64::from(dur.subsec_nanos()) + dur.as_secs() * 1_000_000_000u64;
    println!("{}: sorted {} ints: {} s", "asdasd", 15, nanos as f32 / 1e9f32);

    start = Instant::now();

    for num in 0..45 {
        println!("fib({}) = {}", num, fib_recursive(num));
    }

    dur = Instant::now() - start;
    nanos = u64::from(dur.subsec_nanos()) + dur.as_secs() * 1_000_000_000u64;
    println!("{}: sorted {} ints: {} s", "asdasd", 15, nanos as f32 / 1e9f32);
}

fn fibonacci_split_recursive() {
    fn fib(n: u64) -> u64 {
        use rayon::iter::ParallelIterator;

        rayon::iter::split(n, |n| {
            if n < 2 {
                (n, None)
            } else {
                (n - 2, Some(n - 1))
            }
        })
        .map(fib_recursive)
        .sum()
    }

    let result = fib(46);

    println!("result = {}", result);
}

fn fib_recursive(n: u64) -> u64 {
    if n < 2 {
        return n;
    }

    fib_recursive(n - 1) + fib_recursive(n - 2)
}

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter()
         .map(multiply)
         .sum()
}

fn multiply(input: &i32) -> i32 {

    println!("input = {}", input);

    return input*input;
}