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

    {
        let nan = f32::NAN as u32;
        println!("The value of nan is :{nan}");
        //the maning of the nan is not a number and they are not equal
        assert_eq!(f32::NAN, f32::NAN);
    }
    unsafe {
        // 300.0 as u8 is 44
        println!(" 300.0 as u8 is : {}", 300.0_f32.to_int_unchecked::<u8>());
        // -100.0 as u8 is 156
        println!(
            "-100.0 as u8 is : {}",
            (-100.0_f32).to_int_unchecked::<u8>()
        );
        // nan as u8 is 0
        println!("   nan as u8 is : {}", f32::NAN.to_int_unchecked::<u8>());
    }
}
