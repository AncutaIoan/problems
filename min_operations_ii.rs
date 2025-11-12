/*
2654. Minimum Number of Operations to Make All Array Elements Equal to 1

You are given a 0-indexed array nums consisiting of positive integers. You can do the following operation on the array any number of times:

Select an index i such that 0 <= i < n - 1 and replace either of nums[i] or nums[i+1] with their gcd value.
Return the minimum number of operations to make all elements of nums equal to 1. If it is impossible, return -1.

The gcd of two integers is the greatest common divisor of the two integers.
*/
use std::cmp::min;

impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut ones = 0;
        
        // Count existing 1s
        for &num in &nums {
            if num == 1 {
                ones += 1;
            }
        }
        
        // Case 1: Already have 1s
        if ones > 0 {
            return (n as i32 - ones);
        }

        // Case 2: Need to create a 1
        let mut min_len = n + 1;

        for i in 0..n {
            let mut g = nums[i];
            for j in i+1..n {
                g = gcd(g, nums[j]);
                if g == 1 {
                    min_len = min(min_len, j - i + 1);
                    break;
                }
            }
        }

        if min_len == n + 1 {
            -1
        } else {
            ((min_len - 1) as i32 + (n as i32 - 1))
        }
    }
}

// Helper gcd function
fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}
