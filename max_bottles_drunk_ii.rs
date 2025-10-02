/*
3100. Water Bottles II

You are given two integers numBottles and numExchange.

numBottles represents the number of full water bottles that you initially have. In one operation, you can perform one of the following operations:

Drink any number of full water bottles turning them into empty bottles.
Exchange numExchange empty bottles with one full water bottle. Then, increase numExchange by one.
Note that you cannot exchange multiple batches of empty bottles for the same value of numExchange. For example, if numBottles == 3 and numExchange == 1, you cannot exchange 3 empty water bottles for 3 full bottles.

Return the maximum number of water bottles you can drink.
*/
impl Solution {
    pub fn max_bottles_drunk(mut num_bottles: i32, mut num_exchange: i32) -> i32 {
        let mut total_drunk = 0;
        let mut empty = 0;

        while num_bottles > 0 {
            // Drink all current full bottles
            total_drunk += num_bottles;
            empty += num_bottles;
            num_bottles = 0;

            // Try to exchange if enough empty bottles
            if empty >= num_exchange {
                empty -= num_exchange;
                num_bottles += 1; // gained one full bottle
                num_exchange += 1; // cost increases
            } else {
                break;
            }
        }

        total_drunk
    }
}
