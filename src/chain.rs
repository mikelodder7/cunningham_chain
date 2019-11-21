use gmp::rand::RandState;
use gmp::mpz::{Mpz, ProbabPrimeResult};

use std::thread;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;

use std::collections::LinkedList;
use rand;

use kind::CunninghamKind;
use primes;
use findings::{KNOWN_FIRST_CHAIN, KNOWN_SECOND_CHAIN, KNOWN_BITWIN_CHAIN};

lazy_static! {
    pub static ref ONE: Mpz = { Mpz::from(1) };
    pub static ref TWO: Mpz = { Mpz::from(2) };
    pub static ref THREE: Mpz = { Mpz::from(3) };
    pub static ref FIVE: Mpz = { Mpz::from(5) };
    pub static ref SEVEN: Mpz = { Mpz::from(7) };
    pub static ref EIGHT: Mpz = { Mpz::from(8) };
}

#[derive(Debug, Clone)]
pub struct CunninghamChain {
    bits: usize,
    length: usize,
    kind: CunninghamKind,
    origin: String,
    chain: Vec<String>
}

impl CunninghamChain {
    pub fn make(bits: usize, length: usize, kind: CunninghamKind) -> Result<CunninghamChain, &'static str> {
        let prime_len = (bits as f64 / 10.0_f64.log2()) as i32 + 1;
        println!("Primes with {} digits", prime_len);
        let precheck = (bits as f64).log2() as i32;

        let checks = if prime_len <= 32 {precheck} else {precheck << 1};

        let is_prime= if prime_len <= 64 {
            CunninghamChain::_is_prime_with_factoring
        } else {
            CunninghamChain::_is_prime
        };
        println!("Running {} checks for primality", checks);

        let (tx, rx): (Sender<Result<CunninghamChain, &'static str>>, Receiver<Result<CunninghamChain, &'static str>>) = mpsc::channel();

        let now = ::std::time::Instant::now();
        let func = match kind {
            CunninghamKind::FIRST => CunninghamChain::first,
            CunninghamKind::SECOND => CunninghamChain::second,
            CunninghamKind::BITWIN => CunninghamChain::bi_twin,
        };

        let tx_1 = tx.clone();
        thread::spawn(move || {
            let mut prime_gen = AscendingPrimeGenerator::make(bits);
            println!("Beginning ascending search");
            tx_1.send(func(bits, length, checks, &mut prime_gen, is_prime)).unwrap();
            println!("Finished ascending search");
        });
        for i in 1..4 {
            let seed = CunninghamChain::get_next_seed();
            let tx_i = tx.clone();
            thread::spawn(move || {
                println!("Beginning random {} search with seed={}", i, seed.to_str_radix(10));
                let mut prime_gen = RandomPrimeGenerator::make(bits, seed);
                tx_i.send(func(bits, length, checks, &mut prime_gen, is_prime)).unwrap();
                println!("Finished random {} search", i);
            });
        }
        let tx_2 = tx.clone();
        let seed = CunninghamChain::get_next_seed();
        thread::spawn(move || {
            println!("Beginning primecoin search with seed={}", seed.to_str_radix(10));
            tx_2.send(PrimeCoinChain::make(bits, length, kind, seed)).unwrap();
            println!("Finished primecoin search");
        });
        let result = rx.recv().unwrap();
        println!("Total running time {}", now.elapsed().as_secs());
        result
    }

    fn first<F>(bits: usize, length: usize, checks: i32, prime_gen: &mut dyn PrimeGenerator, is_prime: F) -> Result<CunninghamChain, &'static str> where F: Fn(&Mpz, i32) -> bool {
        let mut primes = LinkedList::new();
        let mut seed = Mpz::from(1) << bits;

        loop {
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
                let t = primes.front().unwrap();
                if KNOWN_FIRST_CHAIN.contains(&t) {
                    println!("Found already known chain {:#?}", CunninghamChain {
                        bits: t.bit_length(),
                        length: primes.len(),
                        origin: t.to_str_radix(10),
                        kind: CunninghamKind::FIRST,
                        chain: primes.iter().map(|p| p.to_str_radix(10)).collect::<Vec<String>>()
                    });
                } else {
                    break;
                }
            }
        }

        let origin = primes.front().unwrap();

        Ok(
            CunninghamChain {
                bits: origin.bit_length(),
                length: primes.len(),
                origin: origin.to_str_radix(10),
                kind: CunninghamKind::FIRST,
                chain: primes.iter().map(|p| p.to_str_radix(10)).collect::<Vec<String>>()
            }
        )
    }

    fn second<F>(bits: usize, length: usize, checks: i32, prime_gen: &mut dyn PrimeGenerator, is_prime: F) -> Result<CunninghamChain, &'static str> where F: Fn(&Mpz, i32) -> bool {
        let mut primes = LinkedList::new();
        let mut seed = Mpz::from(1) << bits;

        loop {
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
                let t = primes.front().unwrap();
                if KNOWN_SECOND_CHAIN.contains(&t) {
                println!("Found already known chain {:#?}", CunninghamChain {
                        bits: t.bit_length(),
                        length: primes.len(),
                        origin: t.to_str_radix(10),
                        kind: CunninghamKind::SECOND,
                        chain: primes.iter().map(|p| p.to_str_radix(10)).collect::<Vec<String>>()
                    });
                } else {
                    break;
                }
            }
        }

        let origin = primes.front().unwrap();

        Ok(
            CunninghamChain {
                bits: origin.bit_length(),
                length: primes.len(),
                origin: origin.to_str_radix(10),
                kind: CunninghamKind::SECOND,
                chain: primes.iter().map(|p| p.to_str_radix(10)).collect::<Vec<String>>()
            }
        )
    }

    fn bi_twin<F>(bits: usize, length: usize, checks: i32, prime_gen: &mut dyn PrimeGenerator, is_prime: F) -> Result<CunninghamChain, &'static str> where F: Fn(&Mpz, i32) -> bool  {
        let mut numbers = LinkedList::new();
        let mut seed = Mpz::from(1) << bits;

        loop {
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
                let t = numbers.front().unwrap();
                if KNOWN_BITWIN_CHAIN.contains(&t) {
                println!("Found already known chain {:#?}", CunninghamChain {
                        bits: seed.bit_length(),
                        length: numbers.len(),
                        origin: t.to_str_radix(10),
                        kind: CunninghamKind::BITWIN,
                        chain: numbers.iter().map(|p| {
                          let mut s = String::from("{");
                          s.push_str(&(p.clone()-1).to_str_radix(10));
                          s.push_str(", ");
                          s.push_str(&(p.clone()+1).to_str_radix(10));
                          s.push('}');
                          s
                          }).collect::<Vec<String>>()
                    });
                } else {
                    break;
                }
            }
        }

        let origin = numbers.front().unwrap();

        Ok(
            CunninghamChain {
                bits: origin.bit_length(),
                length: numbers.len(),
                kind: CunninghamKind::BITWIN,
                origin: origin.to_str_radix(10),
                chain: numbers.iter().map(|p| {
                  let mut s = String::from("{");
                  s.push_str(&(p.clone()-1).to_str_radix(10));
                  s.push_str(", ");
                  s.push_str(&(p.clone()+1).to_str_radix(10));
                  s.push('}');
                  s
                  }).collect::<Vec<String>>()
            }
        )
    }

    fn get_next_seed() -> Mpz {
        let mut acc = Mpz::from(rand::random::<u64>());
        for _ in 1..5 {
            acc <<= rand::random::<u8>() as usize;
            acc ^= Mpz::from(rand::random::<u64>());
        }
        acc
    }

    #[inline]
    fn _is_congruent_to(a: &Mpz, m: &Mpz, e: &Mpz) -> bool {
        a.modulus(m) == *e
    }

    fn _is_prime_with_factoring(n: &Mpz, checks: i32) -> bool {
        for p in primes::PRIMES.iter() {
            if n.is_multiple_of(p) {
                return n == p;
            }
        }

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
            current_seed: TWO.pow(bits as u32 - 1) - 1
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
    pub fn make(bits: usize, seed: Mpz) -> RandomPrimeGenerator {
        let mut rand_state = RandState::new();
        rand_state.seed(seed);
        RandomPrimeGenerator {
            rand_state,
            bits: bits as u64
        }
    }
}

impl PrimeGenerator for RandomPrimeGenerator {
    fn nextprime(&mut self) -> Mpz {
        self.rand_state.urandom_2exp(self.bits).nextprime()
    }
}

struct PrimeCoinChain {
}

impl PrimeCoinChain {
    pub fn make(bits: usize, length: usize, kind: CunninghamKind, seed: Mpz) -> Result<CunninghamChain, &'static str> {
        let mut rand_state = RandState::new();
        rand_state.seed(seed);
        match kind {
            CunninghamKind::FIRST => Self::first(bits, length, &mut rand_state),
            CunninghamKind::SECOND => Self::second(bits, length, &mut rand_state),
            CunninghamKind::BITWIN => Self::bi_twin(bits, length, &mut rand_state),
        }
    }

    fn first(bits: usize, length: usize, rand_state: &mut RandState) -> Result<CunninghamChain, &'static str> {
        let mut primes = LinkedList::new();
        let mut seed;

        loop {
            primes.clear();

            seed = rand_state.urandom_2exp(bits as u64) + 1;

            if !Self::fermat(&seed) {
                continue;
            }

            primes.push_back(seed.clone());

            loop {
                let higher = (primes.back().unwrap() << 1) + 1;

                if Self::fermat(&higher) && Self::euler_lagrange_lifchitz(&higher, true) {
                    primes.push_back(higher);
                } else {
                    let lower = primes.front().unwrap() >> 1;

                    if Self::fermat(&lower) && Self::euler_lagrange_lifchitz(&lower, true) {
                        primes.push_front(lower);
                    } else {
                        break;
                    }
                }
            }

            if primes.len() >= length {
                let t = primes.front().unwrap();
                if KNOWN_FIRST_CHAIN.contains(&t) {
                    println!("Found already known chain {:#?}", CunninghamChain {
                        bits: t.bit_length(),
                        length: primes.len(),
                        origin: t.to_str_radix(10),
                        kind: CunninghamKind::FIRST,
                        chain: primes.iter().map(|p| p.to_str_radix(10)).collect::<Vec<String>>()
                    });
                } else {
                    break;
                }
            }
        }

        let origin = primes.front().unwrap();

        Ok(
            CunninghamChain {
                bits: origin.bit_length(),
                length: primes.len(),
                origin: origin.to_str_radix(10),
                kind: CunninghamKind::FIRST,
                chain: primes.iter().map(|p| p.to_str_radix(10)).collect::<Vec<String>>()
            }
        )
    }

    fn second(bits: usize, length: usize, rand_state: &mut RandState) -> Result<CunninghamChain, &'static str> {
        let mut primes = LinkedList::new();
        let mut seed;

        loop {
            primes.clear();

            seed = rand_state.urandom_2exp(bits as u64) + 1;

            if !Self::fermat(&seed) {
                continue;
            }

            primes.push_back(seed.clone());

            loop {
                let higher = (primes.back().unwrap() << 1) - 1;

                if Self::fermat(&higher) && Self::euler_lagrange_lifchitz(&higher, false) {
                    primes.push_back(higher);
                } else {
                    let lower = (primes.front().unwrap() + 1) >> 1;

                    if Self::fermat(&lower) && Self::euler_lagrange_lifchitz(&lower, false) {
                        primes.push_front(lower);
                    } else {
                        break;
                    }
                }
            }

            if primes.len() >= length {
                let t = primes.front().unwrap();
                if KNOWN_SECOND_CHAIN.contains(&t) {
                    println!("Found already known chain {:#?}", CunninghamChain {
                        bits: t.bit_length(),
                        length: primes.len(),
                        origin: t.to_str_radix(10),
                        kind: CunninghamKind::SECOND,
                        chain: primes.iter().map(|p| p.to_str_radix(10)).collect::<Vec<String>>()
                    });
                } else {
                    break;
                }
            }
        }

        let origin = primes.front().unwrap();

        Ok(
            CunninghamChain {
                bits: origin.bit_length(),
                length: primes.len(),
                origin: origin.to_str_radix(10),
                kind: CunninghamKind::FIRST,
                chain: primes.iter().map(|p| p.to_str_radix(10)).collect::<Vec<String>>()
            }
        )
    }

    fn bi_twin(bits: usize, length: usize, rand_state: &mut RandState) -> Result<CunninghamChain, &'static str> {
        let mut numbers = LinkedList::new();
        let mut seed;

        loop {
            numbers.clear();

            seed = rand_state.urandom_2exp(bits as u64) + 1;

            if Self::fermat(&(seed.clone() - 2)) {
                numbers.push_back(seed.clone() - 1);
            } else if Self::fermat(&(seed.clone() + 2)) {
                numbers.push_back(seed.clone() + 1);
            }

            if numbers.len() > 0 {
                loop {
                    let higher = numbers.back().unwrap() << 1;

                    let second = higher.clone() + 1;
                    let first = higher.clone() - 1;

                    if Self::fermat(&second) && Self::euler_lagrange_lifchitz(&second, false) &&
                       Self::fermat(&first) && Self::euler_lagrange_lifchitz(&first, true) {
                        numbers.push_back(higher);
                    } else {
                        let lower = numbers.front().unwrap() >> 1;

                        let second = lower.clone() + 1;
                        let first = lower.clone() - 1;

                        if Self::fermat(&second) && Self::euler_lagrange_lifchitz(&second, false) &&
                           Self::fermat(&first) && Self::euler_lagrange_lifchitz(&first, true) {
                            numbers.push_front(lower);
                        } else {
                            break;
                        }
                    }
                }
            }

            if numbers.len() >= length {
                let t = numbers.front().unwrap();
                if KNOWN_BITWIN_CHAIN.contains(&t) {
                println!("Found already known chain {:#?}", CunninghamChain {
                        bits: seed.bit_length(),
                        length: numbers.len(),
                        origin: t.to_str_radix(10),
                        kind: CunninghamKind::BITWIN,
                        chain: numbers.iter().map(|p| {
                          let mut s = String::from("{");
                          s.push_str(&(p.clone()-1).to_str_radix(10));
                          s.push_str(", ");
                          s.push_str(&(p.clone()+1).to_str_radix(10));
                          s.push('}');
                          s
                          }).collect::<Vec<String>>()
                    });
                } else {
                    break;
                }
            }
        }

        let origin = numbers.front().unwrap();

        Ok(
            CunninghamChain {
                bits: origin.bit_length(),
                length: numbers.len(),
                kind: CunninghamKind::BITWIN,
                origin: origin.to_str_radix(10),
                chain: numbers.iter().map(|p| {
                  let mut s = String::from("{");
                  s.push_str(&(p.clone()-1).to_str_radix(10));
                  s.push_str(", ");
                  s.push_str(&(p.clone()+1).to_str_radix(10));
                  s.push('}');
                  s
                  }).collect::<Vec<String>>()
            }
        )
    }

    /// Perform Fermat's little theorem on the candidate to determine probable
    /// primality.
    fn fermat(n: &Mpz) -> bool {
        TWO.powm(&(n - 1), n) == *ONE
    }

    // Test probable primality of n = 2p +/- 1 based on Euler, Lagrange and Lifchitz
    fn euler_lagrange_lifchitz(n: &Mpz, sophie_germain: bool) -> bool {
        let e = (n - 1) >> 1;
        let rez = TWO.powm(&e, n);
        let n_mod8 = n.modulus(&EIGHT);
        let mut passed = false;
        if sophie_germain && n_mod8 == *SEVEN { // Euler & Lagrange
            passed = rez == *ONE
        } else if sophie_germain && n_mod8 == *THREE { //Lifchitz
            passed = (rez + 1) == *n
        } else if !sophie_germain && n_mod8 == *FIVE { //Lifchitz
            passed = (rez + 1) == *n
        } else if !sophie_germain && n_mod8 == *ONE { //Lifchitz
            passed = rez == *ONE
        }

        passed
    }
}


trait KindFinder {
    fn get_kind(&self) -> CunninghamKind;
    fn next_seed(&mut self) -> Mpz;
    fn get_higher(&self, seed: &Mpz) -> Mpz;
    fn check_higher<F>(&self, higher: &Mpz) -> bool;
    fn get_lower(&self, seed: &Mpz) -> Mpz;
    fn check_lower<F>(&self, lower: &Mpz) -> bool;
    fn get_result(&self, number: &LinkedList<Mpz>) -> CunninghamChain;
}
