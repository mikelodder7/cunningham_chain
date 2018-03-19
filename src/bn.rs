use openssl::bn::{BigNum, BigNumRef, BigNumContext};
use openssl::error::ErrorStack;

use int_traits::IntTraits;

use std::cmp::Ord;
use std::cmp::Ordering;

pub struct BigNumberContext {
    context: BigNumContext
}

#[derive(Debug)]
pub struct BigNumber {
    bignumber: BigNum
}

lazy_static! {
    pub static ref ONE: BigNumber = { BigNumber::from_u32(1).unwrap() };
}
lazy_static! {
    pub static ref TWO: BigNumber = { BigNumber::from_u32(2).unwrap() };
}
lazy_static! {
    pub static ref THREE: BigNumber = { BigNumber::from_u32(3).unwrap() };
}

impl BigNumber {
    
    pub fn new_context() -> Result<BigNumberContext, ErrorStack> {
        let context = BigNumContext::new()?;
        Ok(BigNumberContext { context })
    }

    pub fn new() -> Result<BigNumber, ErrorStack> {
        let bignumber = BigNum::new()?;
        Ok(BigNumber{ bignumber })
    }

    pub fn generate_prime(size: usize) -> Result<BigNumber, ErrorStack> {
        let mut bn = BigNumber::new()?;
        BigNumRef::generate_prime(&mut bn.bignumber, size as i32, false, None, None)?;
        Ok(bn)
    }

    pub fn generate_safe_prime(size: usize) -> Result<BigNumber, ErrorStack> {
        let mut bn = BigNumber::new()?;
        BigNumRef::generate_prime(&mut bn.bignumber, (size + 1) as i32, true, None, None)?;
        Ok(bn)
    }

    pub fn is_safe_prime(&self, ctx: &mut BigNumberContext) -> Result<bool, ErrorStack> {
        // according to https://eprint.iacr.org/2003/186.pdf
        // we can test if the number is congruent to 2 mod 3
        // if it fails then its not a safe prime
        if self.modulus(&THREE, ctx)?.bignumber == TWO.bignumber && self.is_prime(ctx)? {
            // for "safe prime" generation, check that (p-1)/2 is prime. Since a
            // prime is odd, We just need to divide by 2
            Ok(self.rshift1()?.is_prime(ctx)?)
        } else {
            Ok(false)
        }
    }
    
    pub fn is_prime(&self, ctx: &mut BigNumberContext) -> Result<bool, ErrorStack> {
        let prime_len = self.to_dec()?.len();
        let checks = prime_len.log2() as i32;
        Ok(self.bignumber.is_prime(checks, &mut ctx.context)?)
    }

    pub fn from_u32(n: usize) -> Result<BigNumber, ErrorStack> {
        let bn = BigNum::from_u32(n as u32)?;
        Ok(BigNumber { bignumber: bn })
    }

    pub fn from_dec(dec: &str) -> Result<BigNumber, ErrorStack> {
        let bn = BigNum::from_dec_str(dec)?;
        Ok(BigNumber { bignumber: bn })
    }

    pub fn to_dec(&self) -> Result<String, ErrorStack> {
        let result = self.bignumber.to_dec_str()?;
        Ok(result.to_string())
    }

    pub fn increment(&self) -> Result<BigNumber, ErrorStack> {
        let mut bn = BigNumber::new()?;        
        BigNumRef::checked_add(&mut bn.bignumber, &self.bignumber, &ONE.bignumber)?;
        Ok(bn)
    }

    pub fn decrement(&self) -> Result<BigNumber, ErrorStack> {
        let mut bn = BigNumber::new()?;        
        BigNumRef::checked_sub(&mut bn.bignumber, &self.bignumber, &ONE.bignumber)?;
        Ok(bn)
    }

    pub fn lshift1(&self) -> Result<BigNumber, ErrorStack> {
        let mut bn = BigNumber::new()?;
        BigNumRef::lshift1(&mut bn.bignumber, &self.bignumber)?;
        Ok(bn)
    }

    pub fn rshift1(&self) -> Result<BigNumber, ErrorStack> {
        let mut bn = BigNumber::new()?;
        BigNumRef::rshift1(&mut bn.bignumber, &self.bignumber)?;
        Ok(bn)
    }

    pub fn modulus(&self, a: &BigNumber, ctx: &mut BigNumberContext) -> Result<BigNumber, ErrorStack> {
        let mut bn = BigNumber::new()?;
        BigNumRef::nnmod(&mut bn.bignumber, &self.bignumber, &a.bignumber, &mut ctx.context)?;
        Ok(bn)
    }

    pub fn clone(&self) -> Result<BigNumber, ErrorStack> {
        Ok(BigNumber { bignumber: BigNum::from_slice(&self.bignumber.to_vec())? } )
    }
}

impl Ord for BigNumber {
    fn cmp(&self, other: &BigNumber) -> Ordering {
        self.bignumber.cmp(&other.bignumber)
    }
}

impl Eq for BigNumber {}

impl PartialOrd for BigNumber {
    fn partial_cmp(&self, other: &BigNumber) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BigNumber {
    fn eq(&self, other: &BigNumber) -> bool {
        self.bignumber == other.bignumber
    }
}
