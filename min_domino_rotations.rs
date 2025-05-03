/*
1007. Minimum Domino Rotations For Equal Row

In a row of dominoes, tops[i] and bottoms[i] represent the top and bottom halves of the ith domino. (A domino is a tile with two numbers from 1 to 6 - one on each half of the tile.)

We may rotate the ith domino, so that tops[i] and bottoms[i] swap values.

Return the minimum number of rotations so that all the values in tops are the same, or all the values in bottoms are the same.

If it cannot be done, return -1.
*/



impl Solution {
    pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
        fn check(x: i32, tops: &Vec<i32>, bottoms: &Vec<i32>) -> i32 {
            let mut top_rotations = 0;
            let mut bottom_rotations = 0;

            for i in 0..tops.len() {
                if tops[i] != x && bottoms[i] != x {
                    return -1; // x cannot dominate this domino
                } else if tops[i] != x {
                    top_rotations += 1;
                } else if bottoms[i] != x {
                    bottom_rotations += 1;
                }
            }

            top_rotations.min(bottom_rotations)
        }

        let candidate1 = tops[0];
        let candidate2 = bottoms[0];

        let rotations1 = check(candidate1, &tops, &bottoms);
        if rotations1 != -1 {
            return rotations1;
        }

        if candidate1 != candidate2 {
            let rotations2 = check(candidate2, &tops, &bottoms);
            if rotations2 != -1 {
                return rotations2;
            }
        }

        -1
    }
}


// fn find_majority(arr: &[i32]) -> i32 {
//     let mut candidate = -1;
//     let mut votes = 0;

//     for &num in arr {
//         if votes == 0 {
//             candidate = num;
//             votes = 1;
//         } else if num == candidate {
//             votes += 1;
//         } else {
//             votes -= 1;
//         }
//     }

//     let count = arr.iter().filter(|&&x| x == candidate).count();
//     if count > arr.len() / 2 {
//         candidate
//     } else {
//         -1
//     }
// }

// impl Solution {
//     pub fn min_domino_rotations(tops: Vec<i32>, bottoms: Vec<i32>) -> i32 {
//         if tops.len() != bottoms.len() {
//             return -1;
//         }

//         if tops.len() == 1 {
//             return 0;
//         }

//         // Create combined array for majority check
//         let mut combined = vec![];
//         for i in 0..tops.len() {
//             combined.push(tops[i]);
//             combined.push(bottoms[i]);
//         }

//         let majority = find_majority(&combined);
//         if majority == -1 {
//             return -1;
//         }

//         // Try to make all tops or all bottoms equal to majority
//         let mut top_rotations = 0;
//         let mut bottom_rotations = 0;

//         for i in 0..tops.len() {
//             if tops[i] != majority && bottoms[i] != majority {
//                 return -1; // Can't make majority in this domino
//             } else if tops[i] != majority {
//                 top_rotations += 1;
//             } else if bottoms[i] != majority {
//                 bottom_rotations += 1;
//             }
//         }

//         top_rotations.min(bottom_rotations)
//     }
// }
