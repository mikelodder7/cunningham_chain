use gmp::rand::RandState;
use gmp::mpz::{Mpz, ProbabPrimeResult};

use int_traits::IntTraits;

use std;
use std::io::Write;
use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use std::collections::LinkedList;

use kind::CunninghamKind;
use primes;

lazy_static! {
    pub static ref ONE: Mpz = { Mpz::from(1) };
}

lazy_static! {
    pub static ref TWO: Mpz = { Mpz::from(2) };
}

lazy_static! {
    pub static ref THREE: Mpz = { Mpz::from(3) };
}

#[derive(Debug, Clone)]
pub struct CunninghamChain {
    bits: usize,
    length: usize,
    kind: CunninghamKind,
    starting_prime: String
}

impl CunninghamChain {
    pub fn make(bits: usize, length: usize, kind: CunninghamKind) -> Result<CunninghamChain, &'static str> {
        let prime_len = (bits as i32 / 10.log2()) as i32;
        println!("Primes with {} digits", prime_len);

        let checks = if prime_len <= 32 {bits.log2() as i32} else {bits.log2() as i32 * 2};

        let is_prime= if prime_len <= 32 {
            CunninghamChain::_is_prime_with_factoring
        } else {
            CunninghamChain::_is_prime
        };
        println!("Running {} checks for primality", checks);

        let (tx, rx): (Sender<Result<CunninghamChain, &'static str>>, Receiver<Result<CunninghamChain, &'static str>>) = mpsc::channel();
        let tx_1 = tx.clone();
        let tx_2 = tx.clone();

        let now = ::std::time::Instant::now();
        match kind {
            CunninghamKind::FIRST => {
                thread::spawn(move || {
                    let mut prime_gen = AscendingPrimeGenerator::make(bits);
                    println!("Beginning ascending search");
                    tx_1.send(CunninghamChain::first(bits, length, checks, &mut prime_gen, is_prime)).unwrap();
                    println!("Finished ascending search");
                });
                thread::spawn(move|| {
                    let mut prime_gen = RandomPrimeGenerator::make(bits);
                    println!("Beginning random search");
                    tx_2.send(CunninghamChain::first(bits, length, checks, &mut prime_gen, is_prime)).unwrap();
                    println!("Finished random search");
                });
                }
            ,
            CunninghamKind::SECOND => {
                thread::spawn(move || {
                    let mut prime_gen = AscendingPrimeGenerator::make(bits);
                    println!("Beginning ascending search");
                    tx_1.send(CunninghamChain::second(bits, length, checks, &mut prime_gen, is_prime)).unwrap();
                    println!("Finished ascending search");
                });
                thread::spawn(move || {
                    let mut prime_gen = RandomPrimeGenerator::make(bits);
                    println!("Beginning random search");
                    tx_2.send(CunninghamChain::second(bits, length, checks, &mut prime_gen, is_prime)).unwrap();
                    println!("Finished random search");
                });
            },
            CunninghamKind::BITWIN => {
                thread::spawn(move || {
                    let mut prime_gen = AscendingPrimeGenerator::make(bits);
                    println!("Beginning ascending search");
                    tx_1.send(CunninghamChain::bi_twin(bits, length, checks, &mut prime_gen, is_prime)).unwrap();
                    println!("Finished ascending search");

                });
                thread::spawn(move || {
                    let mut prime_gen = RandomPrimeGenerator::make(bits);
                    println!("Beginning random search");
                    tx_2.send(CunninghamChain::bi_twin(bits, length, checks, &mut prime_gen, is_prime)).unwrap();
                    println!("Finished random search");
                });

            }
        };
        let result = rx.recv().unwrap();
        println!("Total running time {}", now.elapsed().as_secs());
        result
    }

    fn first<F>(bits: usize, length: usize, checks: i32, prime_gen: &mut PrimeGenerator, is_prime: F) -> Result<CunninghamChain, &'static str> where F: Fn(&Mpz, i32) -> bool {
//        let mut random = RandState::new();
        let mut primes = LinkedList::new();
//        let mut attempt = 1;
//        let mut stdout = std::io::stdout();
//        let mut seed = random.urandom_2exp(bits as u64);
        let mut seed = Mpz::from(1) << bits;

//        print!("Attempt ");
        loop {
//            print!("{}", attempt);
//            stdout.flush().unwrap();
            primes.clear();

            if seed.bit_length() > bits + 4 {
                return Err("Unable to find chain");
            }

            seed = prime_gen.nextprime();
            primes.push_back(seed.clone());

            loop {
                let higher = (primes.back().unwrap() << 1) + 1;

                if CunninghamChain::_is_congruent_to(&higher, &THREE, &TWO) &&
                   is_prime(&higher, checks) {
                    primes.push_back(higher);
                } else {
                    let lower = primes.front().unwrap() >> 1;

                    if is_prime(&lower, checks) {
                        primes.push_front(lower);
                    } else {
                        break;
                    }
                }
            }

            if primes.len() >= length {
//                print!("\n");
//                stdout.flush().unwrap();
                break;
            }
//            for _ in 0..attempt.to_string().len() {
//                print!("\x08");
//            }
//            attempt += 1
        }

        let starting_prime = primes.front().unwrap();

        Ok(
            CunninghamChain {
                bits: starting_prime.bit_length(),
                length: primes.len(),
                starting_prime: starting_prime.to_str_radix(10),
                kind: CunninghamKind::FIRST
            }
        )
    }

    fn second<F>(bits: usize, length: usize, checks: i32, prime_gen: &mut PrimeGenerator, is_prime: F) -> Result<CunninghamChain, &'static str> where F: Fn(&Mpz, i32) -> bool {
//        let mut random = RandState::new();
        let mut primes = LinkedList::new();
//        let mut attempt = 1;
//        let mut stdout = std::io::stdout();
//        let mut seed = random.urandom_2exp(bits as u64);
        let mut seed = Mpz::from(1) << bits;

//        print!("Attempt ");
        loop {
//            print!("{}", attempt);
//            stdout.flush().unwrap();
            primes.clear();

            if seed.bit_length() > bits + 4 {
                return Err("Unable to find chain");
            }

            seed = prime_gen.nextprime();

            primes.push_back(seed.clone());

            loop {
                let higher = (primes.back().unwrap() << 1) - 1;

                if CunninghamChain::_is_congruent_to(&higher, &THREE, &ONE) &&
                   is_prime(&higher, checks) {
                   primes.push_back(higher);
                } else {
                    let lower = (primes.front().unwrap() + 1) >> 1;
                    if is_prime(&lower, checks) {
                        primes.push_front(lower);
                    } else {
                        break;
                    }
                }
            }

            if primes.len() >= length {
//                print!("\n");
//                stdout.flush().unwrap();
                break;
            }
//            for _ in 0..attempt.to_string().len() {
//                print!("\x08");
//            }
//            attempt += 1
        }

        let starting_prime = primes.front().unwrap();

        Ok(
            CunninghamChain {
                bits: starting_prime.bit_length(),
                length: primes.len(),
                starting_prime: starting_prime.to_str_radix(10),
                kind: CunninghamKind::SECOND
            }
        )
    }

    fn bi_twin<F>(bits: usize, length: usize, checks: i32, prime_gen: &mut PrimeGenerator, is_prime: F) -> Result<CunninghamChain, &'static str> where F: Fn(&Mpz, i32) -> bool  {
//        let mut random = RandState::new();
        let mut numbers = LinkedList::new();
//        let mut stdout = std::io::stdout();
//        let mut attempt = 1;
//        let mut seed = random.urandom_2exp(bits as u64);
        let mut seed = Mpz::from(1) << bits;

//        print!("Attempt ");
        loop {
//            print!("{}", attempt);
//            stdout.flush().unwrap();
            numbers.clear();

            if seed.bit_length() > bits + 4 {
                return Err("Unable to find chain");
            }

            seed = prime_gen.nextprime();

            if is_prime(&(seed.clone() - 2), checks) {
                numbers.push_back(seed.clone() - 1);
            } else if is_prime(&(seed.clone() + 2), checks) {
                numbers.push_back(seed.clone() + 1);
            }

            if numbers.len() > 0 {
                loop {
                    let higher = numbers.back().unwrap() << 1;

                    if is_prime(&(higher.clone() + 1), checks) &&
                       is_prime(&(higher.clone() - 1), checks) {
                        numbers.push_back(higher);
                    } else {
                        let lower = numbers.front().unwrap() >> 1;

                        if is_prime(&(lower.clone() + 1), checks) &&
                           is_prime(&(lower.clone() - 1), checks) {
                            numbers.push_front(lower);
                        } else {
                            break;
                        }
                    }
                }
            }

            if numbers.len() >= length {
//                print!("\n");
//                stdout.flush().unwrap();
                break;
            }
//            for _ in 0..attempt.to_string().len() {
//                print!("\x08");
//            }
//            attempt += 1
        }
        
        Ok(
            CunninghamChain {
                bits: seed.bit_length(),
                length: numbers.len(),
                kind: CunninghamKind::BITWIN,
                starting_prime: numbers.front().unwrap().to_str_radix(10)
            }
        )
    }

    #[inline]
    fn _is_congruent_to(a: &Mpz, m: &Mpz, e: &Mpz) -> bool {
        a.modulus(m) == *e
    }

    fn _is_prime_with_factoring(n: &Mpz, checks: i32) -> bool {
        !primes::PRIMES.iter().any(|p| n.modulus(&Mpz::from(*p)).is_zero()) ||
        CunninghamChain::_is_prime(n, checks)
    }

    #[inline]
    fn _is_prime(n: &Mpz, checks: i32) -> bool {
        match n.probab_prime(checks) {
            ProbabPrimeResult::Prime => true,
            ProbabPrimeResult::ProbablyPrime => true,
            ProbabPrimeResult::NotPrime => false
        }
    }
}

#[inline]
fn _is_congruent_to(a: &Mpz, m: &Mpz, e: &Mpz) -> bool {
    a.modulus(m) == *e
}

trait PrimeGenerator {
    fn nextprime(&mut self) -> Mpz;
}

struct AscendingPrimeGenerator {
    current_seed: Mpz
}

impl AscendingPrimeGenerator {
    pub fn make(bits: usize) -> AscendingPrimeGenerator {
        AscendingPrimeGenerator {
            current_seed: Mpz::from(2).pow(bits as u32 - 1) - 1
        }
    }
}

impl PrimeGenerator for AscendingPrimeGenerator {
    fn nextprime(&mut self) -> Mpz {
        let temp = self.current_seed.nextprime();
        self.current_seed = temp.clone();
        temp
    }
}

struct RandomPrimeGenerator {
    rand_state: RandState,
    bits: u64
}

impl RandomPrimeGenerator {
    pub fn make(bits: usize) -> RandomPrimeGenerator {
        RandomPrimeGenerator {
            rand_state: RandState::new(),
            bits: bits as u64
        }
    }
}

impl PrimeGenerator for RandomPrimeGenerator {
    fn nextprime(&mut self) -> Mpz {
        self.rand_state.urandom_2exp(self.bits).nextprime()
    }
}

trait KindFinder {
    fn get_higher(&self, seed: &Mpz) -> Mpz;
    fn check_higher<F>(&self, higher: &Mpz, is_prime: F) -> bool where F: Fn(&Mpz, i32) -> bool;
    fn get_lower(&self, seed: &Mpz) -> Mpz;
    fn check_lower<F>(&self, lower: &Mpz, is_prime: F) -> bool where F: Fn(&Mpz, i32) -> bool;
}

struct FirstKind {
    checks: i32
}

impl KindFinder for FirstKind {
    fn get_higher(&self, seed: &Mpz) -> Mpz {
        seed.clone() << 1 + 1
    }
    fn check_higher<F>(&self, higher: &Mpz, is_prime: F) -> bool where F: Fn(&Mpz, i32) -> bool {
        _is_congruent_to(higher, &THREE, &TWO) &&
        is_prime(higher, self.checks)
    }
    fn get_lower(&self, seed: &Mpz) -> Mpz {
        seed.clone() >> 1
    }
    fn check_lower<F>(&self, lower: &Mpz, is_prime: F) -> bool where F: Fn(&Mpz, i32) -> bool {
        is_prime(lower, self.checks)
    }
}

struct SecondKind {
    checks: i32
}

impl KindFinder for SecondKind {
    fn get_higher(&self, seed: &Mpz) -> Mpz {
        seed.clone() << 1 + 1
    }
    fn check_higher<F>(&self, higher: &Mpz, is_prime: F) -> bool where F: Fn(&Mpz, i32) -> bool {
        _is_congruent_to(higher, &THREE, &ONE) &&
        is_prime(higher, self.checks)
    }
    fn get_lower(&self, seed: &Mpz) -> Mpz {
        (seed.clone() + 1) >> 1
    }
    fn check_lower<F>(&self, lower: &Mpz, is_prime: F) -> bool where F: Fn(&Mpz, i32) -> bool {
        is_prime(lower, self.checks)
    }
}

struct BiTwinKind {
    checks: i32
}

impl KindFinder for BiTwinKind {
    fn get_higher(&self, seed: &Mpz) -> Mpz {
        seed.clone() << 1
    }
    fn check_higher<F>(&self, higher: &Mpz, is_prime: F) -> bool where F: Fn(&Mpz, i32) -> bool {
        is_prime(&(higher.clone() + 1), self.checks) &&
        is_prime(&(higher.clone() - 1), self.checks)
    }
    fn get_lower(&self, seed: &Mpz) -> Mpz {
        seed.clone() >> 1
    }
    fn check_lower<F>(&self, lower: &Mpz, is_prime: F) -> bool where F: Fn(&Mpz, i32) -> bool {
        self.check_higher(lower, is_prime)
    }
}

