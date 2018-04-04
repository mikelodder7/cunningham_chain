#!/usr/bin/perl

use v5.18;
say <<"EOF";
use gmp::mpz::Mpz;
use std::collections::HashSet;

macro_rules! hashset {
    ( $( $x:expr ),* ) => {
        {
            let mut set = ::std::collections::HashSet::new();
            $(
                set.insert($x);
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

EOF

open(my $FH, "../findings.md") or die("Unable to find ../findings.md");
binmode($FH);

my $first = "";
my $second = "";
my $bitwin = "";
while (!eof($FH))
{
	my $line = <$FH>;

	my @sections = split(/\|/, $line);

	my $origin = $sections[3];

	if ($sections[0] =~ m/bitwin/io)
	{
	}
	elsif ($sections[0] =~ m/1st/io)
	{
	}
	elsif ($sections[0] =~ m/2nd/io)
	{
	}
}
close($FH);
