/*
2410. Maximum Matching of Players With Trainers

You are given a 0-indexed integer array players, where players[i] represents the ability of the ith player. You are also given a 0-indexed integer array trainers, where trainers[j] represents the training capacity of the jth trainer.

The ith player can match with the jth trainer if the player's ability is less than or equal to the trainer's training capacity. Additionally, the ith player can be matched with at most one trainer, and the jth trainer can be matched with at most one player.

Return the maximum number of matchings between players and trainers that satisfy these conditions.
*/

impl Solution {
    pub fn match_players_and_trainers(mut players: Vec<i32>, mut trainers: Vec<i32>) -> i32 {
        players.sort();
        trainers.sort();

        let mut i = 0; // pointer for players
        let mut j = 0; // pointer for trainers
        let mut matches = 0;

        while i < players.len() && j < trainers.len() {
            if players[i] <= trainers[j] {
                // Match found
                matches += 1;
                i += 1;
                j += 1;
            } else {
                // Trainer not strong enough, try next trainer
                j += 1;
            }
        }

        matches
    }
}
