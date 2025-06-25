/*
2040. Kth Smallest Product of Two Sorted Arrays

Given two sorted 0-indexed integer arrays nums1 and nums2 as well as an integer k, return the kth (1-based) smallest product of nums1[i] * nums2[j] where 0 <= i < nums1.length and 0 <= j < nums2.length.
 
*/

impl Solution {
    pub fn kth_smallest_product(nums1: Vec<i32>, nums2: Vec<i32>, k: i64) -> i64 {
        fn count_less_equal(nums1: &[i32], nums2: &[i32], x: i64) -> i64 {
            let mut count = 0;
            for &a in nums1 {
                if a > 0 {
                    // For positive a, find max b such that a * b <= x
                    let mut l = 0;
                    let mut r = nums2.len();
                    while l < r {
                        let m = (l + r) / 2;
                        if (a as i64) * (nums2[m] as i64) <= x {
                            l = m + 1;
                        } else {
                            r = m;
                        }
                    }
                    count += l as i64;
                } else if a < 0 {
                    // For negative a, find min b such that a * b <= x
                    let mut l = 0;
                    let mut r = nums2.len();
                    while l < r {
                        let m = (l + r) / 2;
                        if (a as i64) * (nums2[m] as i64) <= x {
                            r = m;
                        } else {
                            l = m + 1;
                        }
                    }
                    count += (nums2.len() - l) as i64;
                } else {
                    // a == 0
                    if x >= 0 {
                        count += nums2.len() as i64;
                    }
                }
            }
            count
        }

        let (min1, max1) = (*nums1.first().unwrap() as i64, *nums1.last().unwrap() as i64);
        let (min2, max2) = (*nums2.first().unwrap() as i64, *nums2.last().unwrap() as i64);
        let mut lo = -10_i64.pow(10);
        let mut hi = 10_i64.pow(10);

        while lo < hi {
            let mid = lo + (hi - lo) / 2;
            let count = count_less_equal(&nums1, &nums2, mid);
            if count < k {
                lo = mid + 1;
            } else {
                hi = mid;
            }
        }

        lo
    }
}
