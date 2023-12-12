use std::io;

fn main() -> io::Result<()> {
    println!("Reading from console");
    let mut buffer = String::new();
    let stdin = io::stdin(); // We get `Stdin` here.
    stdin.read_line(&mut buffer)?;
    // operação
    let mut number: u32;

    if let Ok(number) = buffer.trim().parse::<u32>() {
        for i in 0..number{
            println!("{:?}", number);
        }
    }

    //println!("{}", number);
    Ok(())
}
