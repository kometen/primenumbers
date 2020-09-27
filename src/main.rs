fn main() {
    let mut primes: Vec<usize> = vec![2];
    let mut is_prime: bool = false;

    for x in 3..200000 {
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
}
