/*
3354. Make Array Elements Equal to Zero

You are given an integer array nums.

Start by selecting a starting position curr such that nums[curr] == 0, and choose a movement direction of either left or right.

After that, you repeat the following process:

If curr is out of the range [0, n - 1], this process ends.
If nums[curr] == 0, move in the current direction by incrementing curr if you are moving right, or decrementing curr if you are moving left.
Else if nums[curr] > 0:
Decrement nums[curr] by 1.
Reverse your movement direction (left becomes right and vice versa).
Take a step in your new direction.
A selection of the initial position curr and movement direction is considered valid if every element in nums becomes 0 by the end of the process.

Return the number of possible valid selections.
*/

impl Solution {
    pub fn count_valid_selections(nums: Vec<i32>) -> i32 {
        let n = nums.len();
        let mut count = 0;

        // Helper closure to simulate the process
        let simulate = |mut arr: Vec<i32>, mut curr: i32, mut dir: i32| -> bool {
            while curr >= 0 && curr < arr.len() as i32 {
                let idx = curr as usize;
                if arr[idx] == 0 {
                    curr += dir;
                } else {
                    arr[idx] -= 1;
                    dir = -dir;
                    curr += dir;
                }
            }
            arr.iter().all(|&x| x == 0)
        };

        for i in 0..n {
            if nums[i] == 0 {
                if simulate(nums.clone(), i as i32, 1) { // move right
                    count += 1;
                }
                if simulate(nums.clone(), i as i32, -1) { // move left
                    count += 1;
                }
            }
        }

        count
    }
}
