impl Solution {
    pub fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
        let mut p1 = m as isize - 1; // Last valid element in nums1
        let mut p2 = n as isize - 1; // Last element in nums2
        let mut p = (m + n) as isize - 1; // Last position in nums1

        while p1 >= 0 && p2 >= 0 {
            if nums1[p1 as usize] > nums2[p2 as usize] {
                nums1[p as usize] = nums1[p1 as usize];
                p1 -= 1;
            } else {
                nums1[p as usize] = nums2[p2 as usize];
                p2 -= 1;
            }
            p -= 1;
        }

        while p2 >= 0 {
            nums1[p as usize] = nums2[p2 as usize];
            p2 -= 1;
            p -= 1;
        }
    }
}
