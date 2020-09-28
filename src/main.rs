extern crate clap;

use clap::{Arg, App};

fn main() {
    // Command line parameters.
    let matches = App::new("primenumbers")
    .version("0.1")
    .about("calculate primenumbers")
    .author("Claus Guttesen")
    .arg(Arg::with_name("limit")
        .help("upper limit")
        .required(true)
        .takes_value(true)
        .short("l")
        .long("limit")
        .multiple(true)
    )
    .get_matches();

    let limit = match matches.value_of("limit").unwrap().trim().parse::<usize>() {
        Ok(limit) => limit,
        Err(_e) => { 0 }
    };

    let mut primes: Vec<usize> = vec![2];
    let mut is_prime: bool = false;

    for x in (3..limit).step_by(2) {
        for &prime in &primes {
            // Assume it's a prime.
            is_prime = true;
            if &x % &prime == 0 {
                // Divisible by already found primes.
                is_prime = false;
                break;
            }
            if 2 * &prime > x {
                // Stop looping when we get past half the candidate number.
                break;
            }
        }
        if is_prime {
            let _x = x;
            primes.push(_x);
        }
    }
/*    for &prime in &primes {
        println!("{}", prime);
    }*/
//    println!("Number of primenumbers: {}", primes.len());
//    println!("Primenumber at index 1967: {}", primes[1967]);
}
