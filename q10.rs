fn is_prime(num: u64) -> bool {
    if num <= 1 {
        return false; // Numbers less than or equal to 1 are not prime
    }
    // Check if the number is divisible by any integer from 2 to its square root
    for i in 2..=(num as f64).sqrt() as u64 {
        if num % i == 0 {
            return false; 
        }
    }
    true 
}

fn main() {
    let num = 17;
    if is_prime(num) {
        println!("{} is a prime number", num);
    } else {
        println!("{} is not a prime number", num);
    }
}
