fn main() {
    // Problem 1
    let d = distance(5.0, 5.0, 4.0, 4.0);
    println!("Distance = {}", d);

    assert!(is_close(d, 2.0_f64.sqrt()));


    println!("{}", nth_prime(22));
    // Problem 2


}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2-x1).powf(2.0) + (y2-y1).powf(2.0)).sqrt()
}

fn is_close(num1: f64, num2: f64) -> bool {
    (num1 - num2).abs() < 1e-10
}

fn nth_prime(mut n: u32) -> u32 {
    n += 1;
    //brute force
    let mut i: u32 = 2;
    while n > 0 {
        if isPrime(i) {
            n -= 1;
        }
        i += 1;
    }

    i - 1
    // sieving primes
    
}

// Checks if the given number is prime
fn isPrime(num: u32) -> bool {
    // cases for 1, 2, and 3
    if num <= 1 {
        return false;
    }
    if num == 2 || num == 3 {
        return true;
    }

    // if the num is divisible by 2 or 3
    if num % 2 == 0 || num % 3 == 0 {
        return false;
    }

    let mut i = 5;
    while i * i <= num {
        if num % i == 0 || num % (i + 2) == 0 {
            return false;
        }
        i += 6;
    }
    true
}
/*
fn is_taxicab(n: u32) -> bool {




}
*/