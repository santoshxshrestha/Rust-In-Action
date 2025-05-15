#![allow(unused)]
use std::arch::asm;

fn main() {
    {
        unsafe {
            asm!("nop");
        }
    }

    {
        let x: u64;
        unsafe {
            asm!("mov {}, 5",out(reg) x);
        }
        assert_eq!(x,5);
    }

    {

        let i: u64 = 3;
        let o: u64;

        unsafe {
            asm!(
                "mov {0}, {1}",
                "add {0}, 5",
                out(reg) o,
                in(reg) i,
            );
            assert_eq!(0, 8);
        }

    }
}
