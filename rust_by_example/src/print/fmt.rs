pub fn it_works() {
    let number: i32 = 16;

    println!("Number is {}", number);
    println!("Decimal:{}", number);
    println!("Octal:0o{:o}", number);
    println!("Hex:0x{:X}", number);
    println!("Binary:{:b}", number);
    println!("LowerExp:{:e}", number);
    println!("UpperExp:{:E}", number);
    println!("Pointer:{:p}", &number);
    for color in [
        Color { r: 128, g: 255, b: 90 },
        Color { r: 0, g: 3, b: 254 },
        Color { r: 0, g: 0, b: 0 },
    ]
    .iter()
    {
        println!("Display:{}", *color);
        println!("Debug:{:02?}", *color);
        println!("Debug(x?):{:02x?}", *color);
        println!("Debug(X?):{:02X?}", *color);
    }
}
#[derive(Debug)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

use std::fmt::{Display, Formatter, Result};

impl Display for Color {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:02x}{:02x}{:02x}",
            self.r, &self.g, &self.b, &self.r, &self.g, &self.b
        )
    }
}
