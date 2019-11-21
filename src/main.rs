#[macro_use]
extern crate lazy_static;
extern crate gmp;
extern crate rand;

use std::env;

use chain::CunninghamChain;
use kind::CunninghamKind;

mod kind;
mod primes;
mod chain;
mod findings;

fn main() {
    const DEFAULT_BITS: usize = 32;
    const DEFAULT_LENGTH: usize = 3;
    const DEFAULT_KIND: CunninghamKind = CunninghamKind::FIRST;
    let args = env::args().collect::<Vec<String>>();

    let mut bits = DEFAULT_BITS;
    let mut length = DEFAULT_LENGTH;
    let mut kind = DEFAULT_KIND;

    for arg in &args {
        if arg.starts_with("bits=") {
            bits = arg.get(5..).unwrap_or("16").parse::<usize>().expect("Unable to parse bits argument");
        }
        if arg.starts_with("length=") {
            length = arg.get(7..).unwrap_or("2").parse::<usize>().expect("Unable to parse length argument");
        }
        if arg.starts_with("kind=") {
            let k = arg.get(5..).unwrap_or("1").parse::<u32>().expect("Unable to parse kind argument");
            kind = CunninghamKind::from_u32(k).unwrap_or(CunninghamKind::FIRST);
        }
    }

    println!("Finding chain with {} bits and {} length of {:?} kind", bits, length, kind);

    let result = CunninghamChain::make(bits, length, kind);

    match result {
        Ok(r) =>  println!("{:#?}", r),
        Err(m) => println!("{}", m)
    }
    println!("\n");
}
