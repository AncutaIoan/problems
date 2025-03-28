impl Solution {
    pub fn rotate(nums: &mut Vec<i32>, k: i32) {
        let num_elements = nums.len();  // Get the number of elements in the vector
        let k = k % num_elements as i32;  // Ensure k is within the bounds of the vector's size

        // Reverse the entire vector
        nums.reverse();  // This puts the last k elements in front

        // Reverse the first k elements to restore their original order
        nums[0..k as usize].reverse();

        // Reverse the remaining elements to restore their original order
        nums[k as usize..].reverse();
    }
}
