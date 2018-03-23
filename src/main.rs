extern crate openssl;
extern crate int_traits;
#[macro_use] extern crate lazy_static;

use std::env;
use std::io::Write;
use std::collections::LinkedList;

use bn::BigNumber;
use kind::CunninghamKind;

mod bn;
mod kind;

fn main() {
    const DEFAULT_BITS: usize = 16;
    const DEFAULT_LENGTH: usize = 2;
    const DEFAULT_KIND: CunninghamKind = CunninghamKind::FIRST;
    let args = env::args().collect::<Vec<String>>();

    let mut bits = DEFAULT_BITS;
    let mut length = DEFAULT_LENGTH;
    let mut kind = DEFAULT_KIND;

    for arg in &args {
        if arg.starts_with("bits=") {
            bits = arg.get(5..).unwrap_or("16").parse::<usize>().unwrap();
        }
        if arg.starts_with("length=") {
            length = arg.get(7..).unwrap_or("2").parse::<usize>().unwrap();
        }
        if arg.starts_with("kind=") {
            let k = arg.get(5..).unwrap_or("1").parse::<usize>().unwrap(); 
            kind = CunninghamKind::from_u32(k as u32).unwrap_or(CunninghamKind::FIRST);
        }
    }

    println!("Finding chain with {} bits and {} length of {:?} kind", bits, length, kind);

    //TODO: If using ECPP then also print the certificate
    let primes = 
        match kind {
            CunninghamKind::FIRST => first_kind(bits, length),
            CunninghamKind::SECOND => second_kind(bits, length)
        };

    println!("\n");
    println!("Found cunningham {:?} kind of length {}", kind, primes.len());

    //TODO: Possibly offer AKS testing to prove the primes
    let mut index = 1;
    for p in &primes {
        println!("{} = {}", index, p.to_dec().unwrap());
        index += 1;
    }
    println!("\n");
}

fn first_kind(bits: usize, length: usize) -> LinkedList<BigNumber> {
    let mut ctx = BigNumber::new_context().unwrap();
    let mut primes = LinkedList::new();
    let mut attempt = 1;
    let mut stdout = std::io::stdout();

    print!("Attempt ");
    loop {
        print!("{}", attempt);
        stdout.flush().unwrap();
        primes.clear();

        let p = BigNumber::generate_safe_prime(bits).unwrap();
        primes.push_back(p.rshift1().unwrap());
        primes.push_back(p.clone().unwrap());

        loop {
            let higher = primes.back().unwrap().lshift1().unwrap().increment().unwrap();

            if higher.is_safe_prime(&mut ctx).unwrap() {
                primes.push_back(higher);
            } else {
                let lower = primes.front().unwrap().rshift1().unwrap();

                if lower.is_prime(&mut ctx).unwrap() {
                    primes.push_front(lower);
                } else {
                    break;
                }
            }
        }

        if primes.len() >= length {
            print!("\n");
            stdout.flush().unwrap();
            break;
        }
        for _ in 0..attempt.to_string().len() {
            print!("\x08");
        }
        attempt += 1
    }

    primes
}

fn second_kind(bits: usize, length: usize) -> LinkedList<BigNumber> {
    let mut ctx = BigNumber::new_context().unwrap();
    let mut primes = LinkedList::new();
    let mut stdout = std::io::stdout();
    let mut attempt = 1;

    print!("Attempt ");
    loop {
        print!("{}", attempt);
        stdout.flush().unwrap();
        primes.clear();

        let p = BigNumber::generate_prime(bits).unwrap();

        primes.push_back(p.clone().unwrap());

        loop {
            let higher = primes.back().unwrap().lshift1().unwrap().decrement().unwrap();
            if higher.is_prime(&mut ctx).unwrap() {
                primes.push_back(higher);
            } else {
                let lower = primes.front().unwrap().increment().unwrap().rshift1().unwrap();
                if lower.is_prime(&mut ctx).unwrap() {
                    primes.push_front(lower);
                } else {
                    break;
                }
            }
        }

        if primes.len() >= length {
            print!("\n");
            stdout.flush().unwrap();
            break;
        }
        for _ in 0..attempt.to_string().len() {
            print!("\x08");
        }
        attempt += 1
    }

    primes
}
