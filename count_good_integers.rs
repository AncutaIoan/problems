/*
3272. Find the Count of Good Integers
You are given two positive integers n and k.

An integer x is called k-palindromic if:

x is a palindrome.
x is divisible by k.
An integer is called good if its digits can be rearranged to form a k-palindromic integer. For example, for k = 2, 2020 can be rearranged to form the k-palindromic integer 2002, whereas 1010 cannot be rearranged to form a k-palindromic integer.

Return the count of good integers containing n digits.

Note that any integer must not have leading zeros, neither before nor after rearrangement. For example, 1010 cannot be rearranged to form 101.

 

Example 1:

Input: n = 3, k = 5

Output: 27

Explanation:

Some of the good integers are:

551 because it can be rearranged to form 515.
525 because it is already k-palindromic.
Example 2:

Input: n = 1, k = 4

Output: 2

Explanation:

The two good integers are 4 and 8.

Example 3:

Input: n = 5, k = 6

Output: 2468

 

Constraints:

1 <= n <= 10
1 <= k <= 9


*/


use std::collections::HashSet;

impl Solution {
    pub fn count_good_integers(n: i32, k: i32) -> i64 {
        // Precompute factorials for numbers from 0 to n
        let mut factorials = vec![1; (n + 1) as usize];
        for i in 1..=n as usize {
            factorials[i] = factorials[i - 1] * i as i64;
        }

        let mut valid_palindromes_count = 0;
        let mut seen_permutations = HashSet::new();
        let half_base = 10u32.pow(((n - 1) / 2) as u32); // Base for the first half of the palindrome

        for first_half in half_base..half_base * 10 {
            let first_half_str = first_half.to_string();
            let mut reversed_half = first_half_str.clone();
            reversed_half = reversed_half.chars().rev().collect::<String>(); // Reverse the first half
            let mut palindrome = first_half_str.clone();
            palindrome.push_str(&reversed_half[if n % 2 == 0 { 0.. } else { 1.. }]); // Complete the palindrome

            // Check if the palindrome is divisible by k
            if palindrome.parse::<i64>().unwrap() % k as i64 != 0 {
                continue;
            }

            let mut sorted_digits: Vec<char> = palindrome.chars().collect(); // Convert string to char array
            sorted_digits.sort(); // Sort digits to handle unique permutations

            // Skip if the sorted digits have already been processed
            if seen_permutations.contains(&sorted_digits) {
                continue;
            }
            seen_permutations.insert(sorted_digits.clone());

            // Count the frequency of each digit
            let mut digit_counts = vec![0; 10];
            for digit in sorted_digits {
                digit_counts[digit as usize - '0' as usize] += 1;
            }

            // Calculate the number of unique permutations based on digit frequencies
            let mut permutations = (n - digit_counts[0]) as i64 * factorials[n as usize - 1];
            for count in digit_counts {
                permutations /= factorials[count as usize];
            }

            // Add the number of valid permutations to the result
            valid_palindromes_count += permutations;
        }

        valid_palindromes_count
    }
}
