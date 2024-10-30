//  Bitwise operations
//
//  Print binary with modifier :b
//  rotate_left and rotate_right operations
//  count_ones() method
//
fn main() {
    let binary_value:u8 = 100;
    let binary_value_16:u16 = 35500;
    println!("binary_value: {:08b}", binary_value);
    let _ones = binary_value.count_ones();
    let binary_shift = binary_value.rotate_left(1);
    println!("binary_shift: {:08b}", binary_shift);
    println!("binary_value_16: {:016b}", binary_value_16);
}
