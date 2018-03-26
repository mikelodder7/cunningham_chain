extern crate openssl;
extern crate int_traits;
#[macro_use]
extern crate lazy_static;

use std::env;
use std::io::Write;
use std::collections::LinkedList;

use int_traits::IntTraits;

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

    let prime_len = BigNumber::rand(bits).unwrap().to_dec().unwrap().len();
    let do_trial_division = prime_len < 30; //do trial division for numbers less than 30 digits
    let checks = prime_len.log2() as i32;
    //TODO: If using ECPP then also print the certificate
    let primes = 
        match kind {
            CunninghamKind::FIRST => first_kind(bits, length, checks, do_trial_division),
            CunninghamKind::SECOND => second_kind(bits, length, checks, do_trial_division),
            CunninghamKind::BITWIN => bi_twin_kind(bits, length, checks, do_trial_division)
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

fn first_kind(bits: usize, length: usize, checks: i32, do_trial_division: bool) -> LinkedList<BigNumber> {
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

            if higher.is_probably_safe_prime_fast(checks, &mut ctx, do_trial_division).unwrap() {
                primes.push_back(higher);
            } else {
                let lower = primes.front().unwrap().rshift1().unwrap();

                if lower.is_prime_fast(checks, &mut ctx, do_trial_division).unwrap() {
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

fn second_kind(bits: usize, length: usize, checks: i32, do_trial_division: bool) -> LinkedList<BigNumber> {
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
            if higher.is_congruent_to(&bn::THREE, &bn::ONE, &mut ctx).unwrap() &&
               higher.is_prime_fast(checks, &mut ctx, do_trial_division).unwrap() {
               primes.push_back(higher);
            } else {
                let lower = primes.front().unwrap().increment().unwrap().rshift1().unwrap();
                if lower.is_prime_fast(checks, &mut ctx, do_trial_division).unwrap() {
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

fn bi_twin_kind(bits: usize, length: usize, checks: i32, do_trial_division: bool) -> LinkedList<BigNumber> {
    let mut ctx = BigNumber::new_context().unwrap();
    let mut seed;
    let mut right = LinkedList::new();
    let mut left = LinkedList::new();
    let mut stdout = std::io::stdout();
    let mut attempt = 1;

    loop {
        println!("Attempt {}", attempt);
        right.clear();
        left.clear();

        let mut search = 1;
        print!("Seed attempt ");
        loop {
            print!("{}", search);
            stdout.flush().unwrap();

            let safe_p = BigNumber::generate_safe_prime(bits).unwrap();
            let safe_p_2 = safe_p.sub_word(2).unwrap();
            seed = safe_p.rshift1().unwrap();

            if safe_p_2.is_congruent_to(&bn::THREE, &bn::ONE, &mut ctx).unwrap() &&
               safe_p_2.is_prime_fast(checks, &mut ctx, do_trial_division).unwrap() &&
               seed.decrement().unwrap().is_prime_fast(checks, &mut ctx, do_trial_division).unwrap() &&
               seed.increment().unwrap().is_prime_fast(checks, &mut ctx, do_trial_division).unwrap() {

                right.push_back(safe_p);
                left.push_back(safe_p_2);
                break;
            }
            for _ in 0..search.to_string().len() {
                print!("\x08");
            }
            search += 1
        }
        print!("\n");
        stdout.flush().unwrap();

        loop {
            let r = right.back().unwrap().lshift1().unwrap().increment().unwrap();
            let l = left.back().unwrap().lshift1().unwrap().decrement().unwrap();

            if r.is_probably_safe_prime_fast(checks, &mut ctx, do_trial_division).unwrap() &&
               l.is_congruent_to(&bn::THREE, &bn::ONE, &mut ctx).unwrap() &&
               l.is_prime_fast(checks, &mut ctx, do_trial_division).unwrap() {
                right.push_back(r);
                left.push_back(l);
            } else {
                break;
            }
        }

        if right.len() >= length {
            break;
        }
        attempt += 1
    }

    right.push_front(seed);
    right
}
