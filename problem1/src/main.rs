//If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
//Find the sum of all the multiples of 3 or 5 below 1000.

fn main() {
    let count: i32 = (1..1000)
        .filter(|&number| number % 3 == 0 || number % 5 == 0)
        .sum();
    println!("sum of numbers divisible by 3 or 5 is...{count}");
}
