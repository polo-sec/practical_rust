# Hinting Variables in Rust 

## What is Variable Hinting? 
n Rust we'll very often see the compiler complain that our variables and functions aren't type hinted. We saw type hints with CONST earlier. A type hint defines what the data type of a variable is at compile time. E.g.

	let ports: u32 = 65535

## Breakdown

The ": u32" states that the variable ports is of size u32.

The "u" in the integer means unsigned, and the "32" is how many bits it has.

Unsigned integers can only ever be positive, signed integers can be both positive and negative.

Integers range from 16 bits up to 128 bits. Some operating systems can't use integers higher than u32, and using such large integer types may slow down the program on some systems.

