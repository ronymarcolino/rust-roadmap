fn main() 
{
    let b_number: bool = false;
    let c_char: char = 'c';
    let int_number = 100;
    let f_number32: f32 = 3.0;
    let f_number64 = 2.0;
    let s_string = "Hello String";

    println!("Rust basic types:");

    println!("{b_number} ============ (bool)");
    println!("{c_char}  =============== (char)");
    println!("{f_number32:.2} (float with 32 bits)");
    println!("{f_number64:.2} (float with 64 bits)");
    println!("{int_number} ==== (integer number)");
    println!("{s_string} === (string)");
}
