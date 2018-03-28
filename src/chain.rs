use openssl::bn::{BigNum, BigNumRef};
use gmp::rand::RandState;
use gmp::mpz::{Mpz, ProbabPrimeResult};

use int_traits::IntTraits;

use std;
use std::io::Write;

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

        match kind {
            CunninghamKind::FIRST => CunninghamChain::first(bits, length, checks, is_prime),
            CunninghamKind::SECOND => CunninghamChain::second(bits, length, checks, is_prime),
            CunninghamKind::BITWIN => CunninghamChain::bi_twin(bits, length, checks, is_prime)
        }
    }

    fn first<F>(bits: usize, length: usize, checks: i32, is_prime: F) -> Result<CunninghamChain, &'static str> where F: Fn(&Mpz, i32) -> bool {
        let mut seed = BigNum::new().unwrap();
        let mut primes = LinkedList::new();
        let mut attempt = 1;
        let mut stdout = std::io::stdout();

        print!("Attempt ");
        loop {
            print!("{}", attempt);
            stdout.flush().unwrap();
            primes.clear();

            //Generate a safe prime
            CunninghamChain::_generate_safe_prime(&mut seed, bits);

            let p = CunninghamChain::_bignum_to_mpz(&seed);
            primes.push_back(p.clone() >> 1);
            primes.push_back(p);

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
                print!("\n");
                stdout.flush().unwrap();
                break;
            }
            for _ in 0..attempt.to_string().len() {
                print!("\x08");
            }
            attempt += 1
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

    fn second<F>(bits: usize, length: usize, checks: i32, is_prime: F) -> Result<CunninghamChain, &'static str> where F: Fn(&Mpz, i32) -> bool {
        let mut random = RandState::new();
        let mut primes = LinkedList::new();
        let mut attempt = 1;
        let mut stdout = std::io::stdout();
        let mut seed = random.urandom_2exp(bits as u64);

        print!("Attempt ");
        loop {
            print!("{}", attempt);
            stdout.flush().unwrap();
            primes.clear();

            if seed.bit_length() > bits + 4 {
                return Err("Unable to find chain");
            }

            seed = seed.nextprime();

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
                print!("\n");
                stdout.flush().unwrap();
                break;
            }
            for _ in 0..attempt.to_string().len() {
                print!("\x08");
            }
            attempt += 1
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

    fn bi_twin<F>(bits: usize, length: usize, checks: i32, is_prime: F) -> Result<CunninghamChain, &'static str> where F: Fn(&Mpz, i32) -> bool  {
        let mut seed;
        let mut bignum = BigNum::new().unwrap();
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

                CunninghamChain::_generate_safe_prime(&mut bignum, bits);
                let safe_p = CunninghamChain::_bignum_to_mpz(&bignum);
                let safe_p_2 = safe_p.clone() - 2;

                seed = safe_p.clone() >> 1;

                if CunninghamChain::_is_congruent_to(&safe_p_2, &THREE, &ONE) &&
                   is_prime(&safe_p_2, checks) &&
                   is_prime(&(seed.clone() - 1), checks) &&
                   is_prime(&(seed.clone() + 1), checks) {

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
                let r = (right.back().unwrap() << 1) + 1;
                let l = (left.back().unwrap() << 1) - 1;

                if CunninghamChain::_is_congruent_to(&r, &THREE, &TWO) &&
                   is_prime(&r, checks) &&
                   CunninghamChain::_is_congruent_to(&l, &THREE, &ONE) &&
                   is_prime(&l, checks) {
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
        
        Ok(
            CunninghamChain {
                bits: seed.bit_length(),
                length: right.len(),
                kind: CunninghamKind::BITWIN,
                starting_prime: seed.to_str_radix(10)
            }
        )
    }

    #[inline]
    fn _bignum_to_mpz(n: &BigNum) -> Mpz {
        Mpz::from(&n.to_vec()[..])
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

    #[inline]
    fn _generate_safe_prime(number: &mut BigNum, bits: usize) {
        BigNumRef::generate_prime(number, (bits + 1) as i32, true, None, None).unwrap();
    }
}


