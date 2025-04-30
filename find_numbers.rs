/*
1295. Find Numbers with Even Number of Digits

Given an array nums of integers, return how many of them contain an even number of digits.

 

Example 1:

Input: nums = [12,345,2,6,7896]
Output: 2
Explanation: 
12 contains 2 digits (even number of digits). 
345 contains 3 digits (odd number of digits). 
2 contains 1 digit (odd number of digits). 
6 contains 1 digit (odd number of digits). 
7896 contains 4 digits (even number of digits). 
Therefore only 12 and 7896 contain an even number of digits.
Example 2:

Input: nums = [555,901,482,1771]
Output: 1 
Explanation: 
Only 1771 contains an even number of digits.
 

Constraints:

1 <= nums.length <= 500
1 <= nums[i] <= 105


*/


use std::thread;

impl Solution {
    pub fn find_numbers(nums: Vec<i32>) -> i32 {
        let num_threads = 2; // you can increase this
        let chunk_size = (nums.len() + num_threads - 1) / num_threads;

        let mut handles = vec![];

        for chunk in nums.chunks(chunk_size) {
            let chunk = chunk.to_vec(); // move into thread
            let handle = thread::spawn(move || {
                chunk.iter()
                    .filter(|&&n| {
                        let digits = if n == 0 {
                            1
                        } else {
                            (n.abs() as f64).log10().floor() as i32 + 1
                        };
                        digits % 2 == 0
                    })
                    .count() as i32
            });
            handles.push(handle);
        }

        handles.into_iter().map(|h| h.join().unwrap()).sum()
    }
}
