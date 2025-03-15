fn main() {
    let decimal = -42.51_f32;
    let integer: i32 = decimal as i32;
    println!("{integer}");
    let unsigned: u32 = decimal as u32;
    println!("{unsigned}");

    {
        let decimal = 66.23;
        let char_val = decimal as u8 as char;
        println!(" 300.0 as u8 is : {}", 300.0_f32 as u8);
        println!("{char_val}");
    }
}
