#!/usr/bin/perl

use v5.18;

open(my $FH, "findings.md") or die("Unable to find findings.md");
binmode($FH);

my @first = ();
my @second = ();
my @bitwin = ();
while (!eof($FH))
{
	my $line = <$FH>;

	my @sections = split(/\|/, $line);

	my $origin = $sections[4];
  $origin =~ s/^\s+|\s+$//go;

  my $type = $sections[1];

	if ($type =~ m/bitwin/io)
	{
    push(@bitwin, $origin);
	}
	elsif ($type =~ m/1st/io)
	{
    push(@first, $origin);
	}
	elsif ($type =~ m/2nd/io)
	{
    push(@second, $origin);
	}
}

close($FH);
print <<EOF;
use gmp::mpz::Mpz;
use std::collections::HashSet;

macro_rules! hashset {
    ( \$( \$x:expr ),* ) => {
        {
            let mut set = ::std::collections::HashSet::new();
            \$(
                set.insert(\$x);
            )*
            set
        }
    }
}

lazy_static! {
    pub static ref KNOWN_FIRST_CHAIN: HashSet<Mpz> = get_known_first_chain();
}

lazy_static! {
    pub static ref KNOWN_SECOND_CHAIN: HashSet<Mpz> = get_known_second_chain();
}

lazy_static! {
    pub static ref KNOWN_BITWIN_CHAIN: HashSet<Mpz> = get_known_bi_twin_chain();
}

fn get_known_first_chain() -> HashSet<Mpz> {
    hashset![
EOF

my $last_prime = pop(@first);
foreach my $prime (@first) {
  say "        Mpz::from_str_radix(\"$prime\", 10).unwrap(),"
}
print <<EOF;
        Mpz::from_str_radix(\"$last_prime\", 10).unwrap()
    ]
}

fn get_known_second_chain() -> HashSet<Mpz> {
    hashset![
EOF

$last_prime = pop(@second);
foreach my $prime (@second) {
  say "        Mpz::from_str_radix(\"$prime\", 10).unwrap(),"
}
print <<EOF;
        Mpz::from_str_radix(\"$last_prime\", 10).unwrap()
    ]
}

fn get_known_bi_twin_chain() -> HashSet<Mpz> {
    hashset![
EOF

$last_prime = pop(@bitwin);
foreach my $prime (@bitwin) {
  say "        Mpz::from_str_radix(\"$prime\", 10).unwrap(),"
}
say <<EOF;
    ]
}
EOF
