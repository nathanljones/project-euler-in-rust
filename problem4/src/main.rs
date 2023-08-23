/*A palindromic number reads the same both ways.
The largest palindrome made from the product of two -digit numbers is 9009 = 91 x 99.

Find the largest palindrome made from the product of two 3-digit numbers. */

fn is_number_palindrome(number: i32) -> bool {
    let forward_number: String = number.to_string();
    let backward_number: String = forward_number.chars().rev().collect();

    forward_number.eq(&backward_number)
}

fn find_largest_palindrom(no_of_digits: u32) -> i32 {
    let mut max_number: i32 = 0;
    let max_iterations = i32::pow(10, no_of_digits);

    for i in 1..max_iterations {
        for j in 1..max_iterations {
            let possible_palindrome_number = i * j;
            if is_number_palindrome(possible_palindrome_number)
                && possible_palindrome_number > max_number
            {
                max_number = possible_palindrome_number;
            }
        }
    }
    max_number
}

fn main() {
    println!("The answer is {}", find_largest_palindrom(3));
}

#[test]
fn test_is_number_palindrome() {
    assert!(is_number_palindrome(404));
}
#[test]
fn test_is_number_not_palindrome() {
    assert!(!is_number_palindrome(501));
}

#[test]
fn test_largest_palindrome_from_2_digits() {
    assert_eq!(find_largest_palindrom(2), 9009)
}
