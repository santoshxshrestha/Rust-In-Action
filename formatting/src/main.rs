use std::fmt::{self, Display, Formatter};

struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
impl Display for Color {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        // write!(
        //     f,
        //     "RGB ({}, {}, {}) 0x{:0>2X}{:0>2X}{:0>2X}",
        //     self.red, self.green, self.blue, self.red, self.green, self.blue
        // )

        //here is the proper way of doing it which is lot better then the previous mess that I have
        //done
        write!(
            f,
            "RGB ({}, {}, {}) 0x{:06X}",
            self.red,
            self.green,
            self.blue,
            (self.red as u32) << 16 | (self.green as u32) << 8 | (self.blue as u32)
        )
    }
}
fn main() {
    for color in [
        Color {
            red: 128,
            green: 255,
            blue: 90,
        },
        Color {
            red: 0,
            green: 3,
            blue: 254,
        },
        Color {
            red: 0,
            green: 0,
            blue: 0,
        },
    ] {
        println!("{}", color);
    }
}
