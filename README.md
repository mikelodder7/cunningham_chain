# Cunningham Chain Generator
Generate a cunningham chain for 1st, 2nd, and bi-twin kinds.

Cunningham chains have many uses particularly in cryptography.

The program uses GMP to find big primes and test for primality.

Any chains found of sufficient size should be saved in **findings.md**. Contributions are welcome.
Chains in **findings.md** are in the public domain–it is free for use by anyone for any purpose
without restriction under copyright law.

## Building Cunningham Chain Generator
1. Install Rust and rustup (https://www.rust-lang.org/en-US/install.html)
1. Checkout the program:

```
git clone https://github.com/mikelodder7/cunningham_chain.git
```

3. This relies on the openssl crate. See https://crates.io/crates/openssl to install the necessary dependecies.
3. Build the executable

```
cd ./cunningham_chain
cargo build --release
```

5. Rust tests

```
cd ./cunningham_chain
cargo test
```

## Running Cunningham Chain Generator
1. Once the executable is built, the program can be run
```
cargo run bits=N length=L kind=K
```

or

```
./cunningham_chain/target/release/cunningham_chain bits=N length=L kind=K
```

All parameters are optional. Default values are bits=16, length=2, and kind=1
The following are allowed parameters

*bits* - A positive integer. The number of bits to use for generating the primes

*length* A positive integer. The minimum length of the Cunningham chain to find. The program will stop once it finds a chain equal to or longer than this value.

*kind* 1 or 2 or 3. 1 = 1st kind. 2 = 2nd kind. 3 = Bi-Twin
