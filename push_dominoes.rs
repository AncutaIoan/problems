/*
838. Push Dominoes

There are n dominoes in a line, and we place each domino vertically upright. In the beginning, we simultaneously push some of the dominoes either to the left or to the right.

After each second, each domino that is falling to the left pushes the adjacent domino on the left. Similarly, the dominoes falling to the right push their adjacent dominoes standing on the right.

When a vertical domino has dominoes falling on it from both sides, it stays still due to the balance of the forces.

For the purposes of this question, we will consider that a falling domino expends no additional force to a falling or already fallen domino.

You are given a string dominoes representing the initial state where:

dominoes[i] = 'L', if the ith domino has been pushed to the left,
dominoes[i] = 'R', if the ith domino has been pushed to the right, and
dominoes[i] = '.', if the ith domino has not been pushed.
Return a string representing the final state.

 

Example 1:

Input: dominoes = "RR.L"
Output: "RR.L"
Explanation: The first domino expends no additional force on the second domino.


*/

impl Solution {
    pub fn push_dominoes(dominoes: String) -> String {
        let n = dominoes.len();
        let chars: Vec<char> = dominoes.chars().collect();
        let mut forces = vec![0; n];
        
        // Left to right for 'R'
        let mut force = 0;
        for i in 0..n {
            if chars[i] == 'R' {
                force = n as i32; // Max force
            } else if chars[i] == 'L' {
                force = 0;
            } else {
                force = (force - 1).max(0);
            }
            forces[i] += force;
        }
        
        // Right to left for 'L'
        force = 0;
        for i in (0..n).rev() {
            if chars[i] == 'L' {
                force = n as i32;
            } else if chars[i] == 'R' {
                force = 0;
            } else {
                force = (force - 1).max(0);
            }
            forces[i] -= force;
        }
        
        // Build result string
        forces
            .into_iter()
            .map(|f| {
                if f > 0 {
                    'R'
                } else if f < 0 {
                    'L'
                } else {
                    '.'
                }
            })
            .collect()
    }
}
