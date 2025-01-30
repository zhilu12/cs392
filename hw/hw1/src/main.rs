fn main() {
    // Problem 1
    let d = distance_point((5.0, 5.0), (4.0, 4.0));
    println!("Distance = {}", d);

    assert!(is_close(d, 2.0_f64.sqrt()));

    // Problem 2
    println!("22nd prime is {}", nth_prime(22));
    assert!(83 == nth_prime(22));

    // Problem 3
    println!("1729 is a taxicab number: {}", is_taxicab(1729));
    assert!(true == is_taxicab(1729));
}

fn distance(point1: (f64, f64), point2: (f64, f64)) -> f64 {
    ((point2.0 - point1.0).powf(2.0) + (point2.1 - point1.1).powf(2.0)).sqrt()
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

    // sieving primes maybe later
}

// Checks if the given number is prime
fn is_prime(num: u32) -> bool {
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

given a nonnegative integer n of type u32, determines if it can be
represented as the sum of a pair of positive cubes is more than one way.

*/
fn is_taxicab(n: u32) -> bool {
    let mut count = 0;

    let max = (n as f64).cbrt() as u32;

    for i in 0..=max {
        for j in i..=max {
            if (i.pow(3) + j.pow(3)) == n {
                count += 1;
                if count > 1 {
                    return true;
                }
            }
        }
    }

    false
}
