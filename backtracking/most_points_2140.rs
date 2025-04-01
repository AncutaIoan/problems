/*
2140. Solving Questions With Brainpower

You are given a 0-indexed 2D integer array questions where questions[i] = [pointsi, brainpoweri].

The array describes the questions of an exam, where you have to process the questions in order (i.e., starting from question 0) and make a decision whether to solve or skip each question. Solving question i will earn you pointsi points but you will be unable to solve each of the next brainpoweri questions. If you skip question i, you get to make the decision on the next question.

For example, given questions = [[3, 2], [4, 3], [4, 4], [2, 5]]:
If question 0 is solved, you will earn 3 points but you will be unable to solve questions 1 and 2.
If instead, question 0 is skipped and question 1 is solved, you will earn 4 points but you will be unable to solve questions 2 and 3.
Return the maximum points you can earn for the exam.

 

Example 1:

Input: questions = [[3,2],[4,3],[4,4],[2,5]]
Output: 5
Explanation: The maximum points can be earned by solving questions 0 and 3.
- Solve question 0: Earn 3 points, will be unable to solve the next 2 questions
- Unable to solve questions 1 and 2
- Solve question 3: Earn 2 points
Total points earned: 3 + 2 = 5. There is no other way to earn 5 or more points.
Example 2:

Input: questions = [[1,1],[2,2],[3,3],[4,4],[5,5]]
Output: 7
Explanation: The maximum points can be earned by solving questions 1 and 4.
- Skip question 0
- Solve question 1: Earn 2 points, will be unable to solve the next 2 questions
- Unable to solve questions 2 and 3
- Solve question 4: Earn 5 points
Total points earned: 2 + 5 = 7. There is no other way to earn 7 or more points.
 

Constraints:

1 <= questions.length <= 105
questions[i].length == 2
1 <= pointsi, brainpoweri <= 105

*/

use std::cmp;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let n = questions.len();
        let mut dp = vec![0; n + 1]; // DP table with extra space for ease

        for i in (0..n).rev() {
            let points = questions[i][0] as i64;
            let next_question = i + 1 + questions[i][1] as usize;
            
            let take = points + if next_question < n { dp[next_question] } else { 0 };
            let skip = dp[i + 1];

            dp[i] = cmp::max(take, skip);
        }

        dp[0] // Maximum points starting from question 0
    }
}



/*

Naive backtracking solution using caching in case we skip or reach an already computed value




use std::cmp;

impl Solution {
    pub fn most_points(questions: Vec<Vec<i32>>) -> i64 {
        let mut cache = vec![-1; questions.len()]; // Use -1 as uncomputed marker

        fn helper(i: usize, questions: &Vec<Vec<i32>>, cache: &mut Vec<i64>) -> i64 {
            if i >= questions.len() {
                return 0;
            }

            if cache[i] != -1 {
                return cache[i];
            }

            cache[i] = cmp::max(
                questions[i][0] as i64 + helper(i + 1 + questions[i][1] as usize, questions, cache), 
                helper(i + 1, questions, cache)
            );

            cache[i]
        }

        helper(0, &questions, &mut cache)
    }
}
*/
