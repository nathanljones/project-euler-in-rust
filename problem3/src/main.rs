//The prime factors of 13195 are 5, 7, 13 and 29.
//What is the largest prime factor of the number 600851475143 ?

fn main() {
    if is_number_prime(7) {
        println!("7 is prime",);
    } else {
        println!("7 is NOT prime",);
    }
    println!(
        "Lowest prime factor of 12 is {}",
        find_lowest_prime_factor(12)
    );
    println!(
        "largest prime factor of 600851475143 is {}",
        find_largest_prime_factor(600851475143)
    );
}

fn find_largest_prime_factor(number: u64) -> u64 {
    let mut largest_factor: u64 = 0;
    let sqrt_of_number = number as f64;
    let real_sqrt_of_number = sqrt_of_number.sqrt().abs() as u64;
    for counter in 2..real_sqrt_of_number {
        if is_number_prime(counter) && number % counter == 0 {
            largest_factor = counter;
        }
    }
    largest_factor
}
fn find_lowest_prime_factor(number: u64) -> u64 {
    let sqrt_of_number = number as f64;
    let real_sqrt_of_number = sqrt_of_number.sqrt().abs() as u64;
    for divisor in 2..real_sqrt_of_number {
        if is_number_prime(divisor) && number % divisor == 0 {
            return divisor;
        }
    }
    number
}
fn is_number_prime(number_to_check: u64) -> bool {
    let sqrt_of_number = number_to_check as f64;
    let real_sqrt_of_number = sqrt_of_number.sqrt().abs() as u64;
    for counter in 2..real_sqrt_of_number {
        if number_to_check % counter == 0 {
            return false;
        }
    }

    true
}
