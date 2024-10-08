//Each new term in the Fibonacci sequence is generated by adding the previous two terms. By starting with 1 and 2, the first 10 terms will be:
// 1, 2, 3, 5, 8, 13, 21, 34, 55, 89, ...
// By considering the terms in the Fibonacci sequence whose values do not exceed four million, find the sum of the even-valued terms.

fn main() {
    fibonacci(1, 2);
}
fn fibonacci(first_number: u32, second_number: u32) {
    let mut running_total = second_number;

    let mut temp_no1 = first_number;
    let mut temp_no2 = second_number;
    let mut temp_no: u32 = 0;
    while temp_no < 4_000_000 {
        temp_no = temp_no1 + temp_no2;
        if temp_no % 2 == 0 {
            running_total += temp_no;
        }
        temp_no1 = temp_no2;
        temp_no2 = temp_no;
    }
    println!("Total = {running_total}");
}
