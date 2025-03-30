/*
Given a positive integer n, write a function that returns the number of set bits in its binary representation (also known as the Hamming weight).
*/

impl Solution {
    pub fn hamming_weight(n: i32) -> i32 {
        let mut count = 0;
        let mut num = n;

        while num != 0 {
            num &= num - 1;
            count += 1;
        }

        count
    }
}

