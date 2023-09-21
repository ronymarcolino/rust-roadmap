fn main() {
    let number = 0;
    /*
    let text = match number {
        0 => "zero",
        1 => "one",
        2 => "two",
        _ => "many",
    };*/
    let text = match number {
        0..=3 => "small",
        4..=6 => "medium",
        _ => "large",
     };

    println!("text: {}", text);
    /*
    let mut result:u64 = 0;
    let value:u64 = 10;
    result = factorial(value);
    println!("factorial of {} is {}", value, result);
    */
}
/*
fn factorial(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorial(n-1)
    }
}
*/

