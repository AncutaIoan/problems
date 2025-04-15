use std::collections::HashMap;
/*
2179. Count Good Triplets in an Array

You are given two 0-indexed arrays nums1 and nums2 of length n, both of which are permutations of [0, 1, ..., n - 1].

A good triplet is a set of 3 distinct values which are present in increasing order by position both in nums1 and nums2. In other words, if we consider pos1v as the index of the value v in nums1 and pos2v as the index of the value v in nums2, then a good triplet will be a set (x, y, z) where 0 <= x, y, z <= n - 1, such that pos1x < pos1y < pos1z and pos2x < pos2y < pos2z.

Return the total number of good triplets.

 

Example 1:

Input: nums1 = [2,0,1,3], nums2 = [0,1,2,3]
Output: 1
Explanation: 
There are 4 triplets (x,y,z) such that pos1x < pos1y < pos1z. They are (2,0,1), (2,0,3), (2,1,3), and (0,1,3). 
Out of those triplets, only the triplet (0,1,3) satisfies pos2x < pos2y < pos2z. Hence, there is only 1 good triplet.
Example 2:

Input: nums1 = [4,0,1,3,2], nums2 = [4,1,0,2,3]
Output: 4
Explanation: The 4 good triplets are (4,0,3), (4,0,2), (4,1,3), and (4,1,2).
 

Constraints:

n == nums1.length == nums2.length
3 <= n <= 105
0 <= nums1[i], nums2[i] <= n - 1
nums1 and nums2 are permutations of [0, 1, ..., n - 1].


*/

impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let n = nums1.len();

        // Map values in nums2 to their indices
        let mut pos = vec![0; n];
        for (i, &val) in nums2.iter().enumerate() {
            pos[val as usize] = i;
        }

        // Convert nums1 into their corresponding indices in nums2
        let mut transformed = vec![0; n];
        for i in 0..n {
            transformed[i] = pos[nums1[i] as usize];
        }

        // Fenwick Tree / BIT helper for counting
        struct BIT {
            tree: Vec<i32>,
            size: usize,
        }

        impl BIT {
            fn new(n: usize) -> Self {
                Self {
                    tree: vec![0; n + 2],
                    size: n + 2,
                }
            }

            fn update(&mut self, mut index: usize, value: i32) {
                index += 1;
                while index < self.size {
                    self.tree[index] += value;
                    index += index & (!index + 1);
                }
            }

            fn query(&self, mut index: usize) -> i32 {
                let mut res = 0;
                index += 1;
                while index > 0 {
                    res += self.tree[index];
                    index -= index & (!index + 1);
                }
                res
            }
        }

        // Count of elements to the left that are less than current
        let mut left = vec![0; n];
        let mut bit = BIT::new(n);
        for i in 0..n {
            left[i] = bit.query(transformed[i]);
            bit.update(transformed[i], 1);
        }

        // Count of elements to the right that are greater than current
        let mut right = vec![0; n];
        let mut bit = BIT::new(n);
        for i in (0..n).rev() {
            right[i] = bit.query(n - 1) - bit.query(transformed[i]);
            bit.update(transformed[i], 1);
        }

        // Calculate result
        let mut count: i64 = 0;
        for i in 0..n {
            count += left[i] as i64 * right[i] as i64;
        }

        count
    }
}


/*
TIME LIMIT EXCEEDED SOLUTION
impl Solution {
    pub fn good_triplets(nums1: Vec<i32>, nums2: Vec<i32>) -> i64 {
        let map1: HashMap<i32, usize> = nums1
            .iter()
            .enumerate()
            .map(|(i, &num)| (num, i))
            .collect();

        let map2: HashMap<i32, usize> = nums2
            .iter()
            .enumerate()
            .map(|(i, &num)| (num, i))
            .collect();

        let n = nums1.len();
        let mut count = 0;

        for &i in &nums1 {
            let idx1 = *map1.get(&i).unwrap();
            let idx2 = *map2.get(&i).unwrap();


                for &j in &nums1 {
                    let jdx1 = *map1.get(&j).unwrap();
                    let jdx2 = *map2.get(&j).unwrap();

                    if idx1 < jdx1 && idx2 < jdx2 {
                        for &z in &nums1 {
                            let zdx1 = *map1.get(&z).unwrap();
                            let zdx2 = *map2.get(&z).unwrap();

                            if jdx1 < zdx1 && jdx2 < zdx2 {
                                count += 1;
                            }
                        }
                    }
            }
        }

        count
    }
}
*/
