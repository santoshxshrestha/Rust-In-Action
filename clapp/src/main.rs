#![allow(unused)]
use clap::Parser;

#[derive(Parser, Debug)]
struct Args {
    input: String,
    #[arg(short, long)]
    args1: String,
    args2: String,

    #[arg(short, long)]
    bargs3: String,
    args4: String,
}

fn main() {
    let args = Args::parse();
    println!("{}", args.args1);
    println!("{}", args.args2);
    println!("{}", args.bargs3);
    println!("{}", args.args4);
}

//here below is the refrence of the thing
// 󰣇 santosh in Rust-In-Action/clapp 󰘬 main ? -> cargo run -- hello --args1 one two --b three four]
//    Compiling clapp v0.1.0 (/home/santosh/dev/Rust-In-Action/clapp)
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.29s
//      Running `target/debug/clapp hello --args1 one two --b three 'four]'`
// error: unexpected argument '--b' found
//
//   tip: a similar argument exists: '--bargs3'
//
// Usage: clapp --args1 <ARGS1> --bargs3 <BARGS3> <INPUT> <ARGS2> <ARGS4>
//
// For more information, try '--help'.
//
// 󰣇 santosh in Rust-In-Action/clapp 󰘬 main ? ✖ cargo run -- hello --args1 one two -b three four]
//     Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.02s
//      Running `target/debug/clapp hello --args1 one two -b three 'four]'`
// one
// two
// three
// four]
