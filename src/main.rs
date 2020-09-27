fn main() {
    let mut primes = vec![2];
    let mut is_prime: bool = false;

    for x in 3..200 {
//        println!("candidate: {}", x);
        for &prime in &primes {
            is_prime = true;
//            println!("prime: {}", &prime);
            if &x % &prime == 0 {
//                println!("divisible by {}", &prime);
                is_prime = false;
                break;
            }
            if 2 * &prime > x {
//                println!("stop");
                break;
            }
        }
        if is_prime {
            let _x = x;
            primes.push(_x);
        }
    }
    println!("Number of primenumbers: {}", primes.len());
    for &prime in &primes {
        println!("{}", prime);
    }
}


/*

        for (index, &prime) in primes.iter().enumerate() {
            println!("{}", &prime);
            if 2 * x > prime {
                println!("stop");
            }
        }

        for &prime in &primes {
            println!("prime: {}", &prime);
            if 2 * &x < prime {
                println!("continue");
            } else if &x % &prime != 0 {
                println!("continue");
            } else {
                println!("stop");
            }
        }


*/