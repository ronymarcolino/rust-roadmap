fn main() 
{
    let mut count = 0u32;
    println!("Let's count until five!");
    // Using loop like Infinite loop
    loop {
        count += 1;
        if count == 3 
        {
            println!("three");
            // Skip the rest of this iteration
            continue;
        }
        println!("{}", count);
        if count == 5 
        {
            println!("OK, that's enough");
            // Exit this loop
            break;
        }
    }

    println!("Let's write FizzBuzz with while");
    // A counter variable using while loop
    let mut n = 1;
    // Loop while `n` is less than 101
    while n < 101 
    {
        if n % 15 == 0 
        {
            println!("fizzbuzz");
        } else if n % 3 == 0 
        {
            println!("fizz");
        } else if n % 5 == 0 
        {
            println!("buzz");
        } else 
        {
            println!("{}", n);
        }
        // Increment counter
        n += 1;
    }

    println!("Let's write FizzBuzz with for");
    // A counter variable using for loop
    // `n` will take the values: 1, 2, ..., 100 in each iteration
    for n in 1..101 
    {
        if n % 15 == 0 
        {
            println!("fizzbuzz");
        } else if n % 3 == 0 
        {
            println!("fizz");
        } else if n % 5 == 0 
        {
            println!("buzz");
        } else 
        {
            println!("{}", n);
        }
    }
}
