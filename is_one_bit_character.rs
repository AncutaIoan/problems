/*
717. 1-bit and 2-bit Characters

We have two special characters:

The first character can be represented by one bit 0.
The second character can be represented by two bits (10 or 11).
Given a binary array bits that ends with 0, return true if the last character must be a one-bit character.

 
*/
impl Solution {
    pub fn is_one_bit_character(bits: Vec<i32>) -> bool {
        let mut i = 0;
        let n = bits.len();

        while i < n - 1 {
            if bits[i] == 1 {
                i += 2; // two-bit character
            } else {
                i += 1; // one-bit character
            }
        }

        // If we stop exactly at the last index, it's a one-bit character
        i == n - 1
    }
}
