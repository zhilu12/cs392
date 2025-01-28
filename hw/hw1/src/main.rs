fn main() {
    // Problem 1
    let d = distance(5.0, 5.0, 4.0, 4.0);
    println!("Distance = {}", d);

    assert!(is_close(d, 2.0_f64.sqrt()));



    // Problem 2


}

fn distance(x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    ((x2-x1).powf(2.0) + (y2-y1).powf(2.0)).sqrt()
}

fn is_close(num1: f64, num2: f64) -> bool {
    (num1 - num2).abs() < 1e-10
}

fn nth_prime(n: u32) -> u32 {


    
}