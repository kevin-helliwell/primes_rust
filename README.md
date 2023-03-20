# What This Does
Counts and lists primes up to a given number in Rust.

# How It Works
The function determining whether a given number is prime is utilizing the following:

(1) Sieve of Eratosthenes - Eliminate all even numbers first, only check odd numbers for primes.

(2) Square Root Optimization - Only check for factors up to and including the floor of the square root of the given number.

Combining these two, the function only ever checks the odd factors of a given number up to its square root.

# Example

Number = 97

Using a brute force approach, we would have to (worst case) check every number up to 97.

Utilizing the Square Root Optimization, we only have to check numbers 1-9 to see if they divide 97.

Floor(Sqrt(97)) = 9 -> Numbers to check: 1, 2, 3, 4, 5, 6, 7, 8, 9

Utilizing the Sieve of Eratosthenes, we can reduce this even further by eliminating all the even numbers.

Numbers to check: 1, 2, 3, 4, 5, 6, 7, 8, 9 -> Odd Numbers: 1, 3, 5, 7, 9

Only 1 divides 97. This implies 97 is prime.

With these optimizations in place, we only had to check 5 numbers, compared to ~96 if we had used a brute force approach.
