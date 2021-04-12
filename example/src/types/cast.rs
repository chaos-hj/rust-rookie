pub fn it_works() {
    let decimal = 65.432_1_f32;
    let integer = decimal as u8;
    let character = integer as char;

    println!("Casting: {} -> {} -> {}", decimal, integer, character);

    println!("1000 mod 256 is : {}", 1000 % 256);
    println!("1000_i16 as a u8 is : {}", 1000i16 as u8);

    println!("  -1_i8 as a u8 is : {}", (-1i8) as u8);
    println!(" 128 as a i16 is: {}", 128 as i16);
    println!(" 128_u8 as a i8 is : {}", 128u8 as i8);
    println!(" 232_u8 as a i8 is : {}", 232u8 as i8);
}
