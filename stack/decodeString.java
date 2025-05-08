import java.util.Stack;
 

class Solution {
    /*

    3[a2[c]]
    3 * (a + 2 * c)
    3 * (acc)
    accaccacc

    *
    3 +
      a *
        2 c 
        
394. Decode String

Given an encoded string, return its decoded string.

The encoding rule is: k[encoded_string], where the encoded_string inside the square brackets is being repeated exactly k times. Note that k is guaranteed to be a positive integer.

You may assume that the input string is always valid; there are no extra white spaces, square brackets are well-formed, etc. Furthermore, you may assume that the original data does not contain any digits and that digits are only for those repeat numbers, k. For example, there will not be input like 3a or 2[4].

The test cases are generated so that the length of the output will never exceed 105.

 

Example 1:

Input: s = "3[a]2[bc]"
Output: "aaabcbc"
Example 2:

Input: s = "3[a2[c]]"
Output: "accaccacc"
Example 3:

Input: s = "2[abc]3[cd]ef"
Output: "abcabccdcdcdef"
    */

    public String decodeString(String s) {
        Stack<StringBuilder> strStack = new Stack<>();
        Stack<Integer> numStack = new Stack<>();
        StringBuilder currStr = new StringBuilder();
        int num = 0;

        for (char ch : s.toCharArray()) {
            if (Character.isDigit(ch)) {
                num = num * 10 + (ch - '0');
            } else if (ch == '[') {
                strStack.push(currStr);
                numStack.push(num);
                currStr = new StringBuilder();
                num = 0;
            } else if (ch == ']') {
                // Pop from stacks and build repeated string
                StringBuilder prevStr = strStack.pop();
                int repeat = numStack.pop();
                StringBuilder temp = new StringBuilder(prevStr);
                for (int i = 0; i < repeat; i++) {
                    temp.append(currStr);
                }
                currStr = temp;
            } else {
                // Build the current string
                currStr.append(ch);
            }
        }

        return currStr.toString();

    }
}
