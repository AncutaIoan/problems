/*
3307. Find the K-th Character in String Game II


Alice and Bob are playing a game. Initially, Alice has a string word = "a".

You are given a positive integer k. You are also given an integer array operations, where operations[i] represents the type of the ith operation.

Now Bob will ask Alice to perform all operations in sequence:

If operations[i] == 0, append a copy of word to itself.
If operations[i] == 1, generate a new string by changing each character in word to its next character in the English alphabet, and append it to the original word. For example, performing the operation on "c" generates "cd" and performing the operation on "zb" generates "zbac".
Return the value of the kth character in word after performing all the operations.

Note that the character 'z' can be changed to 'a' in the second type of operation.
*/
impl Solution {
    pub fn kth_character(k: i64, operations: Vec<i32>) -> char {
        let mut count_ops = 0;
        let mut val = k;

        while val > 1 {
            let jumps = (val as f64).log2().ceil() as usize;
            val -= 1 << (jumps - 1);
            count_ops += operations[jumps - 1];
        }

        ((b'a' + (count_ops % 26) as u8) as char)
    }
}
