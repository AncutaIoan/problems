use std::vec::Vec;

impl Solution {
    pub fn single_number(nums: Vec<i32>) -> Vec<i32> {
        // Step 1: Compute XOR of all numbers in the array
        let mut total_xor = 0;
        for &num in &nums {
            total_xor ^= num;
        }

        // Step 2: Find the rightmost set bit (bit that differentiates the two unique numbers)
        let last_bit = total_xor & (-total_xor);

        // Step 3: Find one of the unique numbers by XORing numbers that have this bit set
        let mut num1 = 0;
        for &num in &nums {
            if num & last_bit != 0 {
                num1 ^= num;
            }
        }

        // Step 4: Compute the second unique number using XOR with total_xor
        let num2 = total_xor ^ num1;

        // Step 5: Return the two unique numbers
        vec![num1, num2]
    }
}
