/*
3304. Find the K-th Character in String Game I

Alice and Bob are playing a game. Initially, Alice has a string word = "a".

You are given a positive integer k.

Now Bob will ask Alice to perform the following operation forever:

Generate a new string by changing each character in word to its next character in the English alphabet, and append it to the original word.
For example, performing the operation on "c" generates "cd" and performing the operation on "zb" generates "zbac".

Return the value of the kth character in word, after enough operations have been done for word to have at least k characters.

Note that the character 'z' can be changed to 'a' in the operation.
*/
impl Solution {
    pub fn kth_character(k: i32) -> char {
        let mut my_string = String::from("a");
        let k_u = k as usize;

        while my_string.len() < k_u {
            // Create a new string by incrementing each character by 1
            let new_string: String = my_string
                .chars()
                .map(|c| ((c as u8) + 1) as char)
                .collect();

            my_string.push_str(&new_string);
        }

        // k is 1-based, convert to zero-based index
        my_string.chars().nth(k_u - 1).unwrap()
    }
}
