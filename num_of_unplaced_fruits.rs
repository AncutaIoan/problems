/*
3477. Fruits Into Baskets II

You are given two arrays of integers, fruits and baskets, each of length n, where fruits[i] represents the quantity of the ith type of fruit, and baskets[j] represents the capacity of the jth basket.

From left to right, place the fruits according to these rules:

Each fruit type must be placed in the leftmost available basket with a capacity greater than or equal to the quantity of that fruit type.
Each basket can hold only one type of fruit.
If a fruit type cannot be placed in any basket, it remains unplaced.
Return the number of fruit types that remain unplaced after all possible allocations are made.

*/
impl Solution {
    pub fn num_of_unplaced_fruits(fruits: Vec<i32>, baskets: Vec<i32>) -> i32 {
        let n = fruits.len();
        let mut used = vec![false; n];
        let mut unplaced = 0;

        for fruit in fruits.iter() {
            let mut placed = false;
            for (j, &basket) in baskets.iter().enumerate() {
                if !used[j] && basket >= *fruit {
                    used[j] = true;
                    placed = true;
                    break;
                }
            }
            if !placed {
                unplaced += 1;
            }
        }

        unplaced
    }
}
