impl Solution {
    pub fn majority_element(nums: Vec<i32>) -> i32 {
        let mut candidate = 0;
        let mut count = 0;

        // Boyer-Moore Voting Algorithm
        for num in nums {
            if count == 0 {
                candidate = num;
                count = 1;
            } else if num == candidate {
                count += 1;
            } else {
                count -= 1;
            }
        }

        candidate
    }
}
