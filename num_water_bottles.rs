/*
1518. Water Bottles

There are numBottles water bottles that are initially full of water. You can exchange numExchange empty water bottles from the market with one full water bottle.

The operation of drinking a full water bottle turns it into an empty bottle.

Given the two integers numBottles and numExchange, return the maximum number of water bottles you can drink.
*/
impl Solution {
    pub fn num_water_bottles(mut num_bottles: i32, num_exchange: i32) -> i32 {
        let mut sum = 0;
        
        // Drink all current bottles
        sum += num_bottles;

        // Keep exchanging while possible
        while num_bottles >= num_exchange {
            let new_bottles = num_bottles / num_exchange;
            let remainder = num_bottles % num_exchange;

            sum += new_bottles; // drink new bottles
            num_bottles = new_bottles + remainder; // update bottles
        }

        sum
    }
}
