Problem 115. Distinct Subsequences (Hard)
=========================================

See [on LeetCode](https://leetcode.com/problems/distinct-subsequences/).

Given two strings `s` and `t`, return the number of distinct subsequences of `s` which equals `t`.

A string's subsequence is a new string formed from the original string by deleting some (can be none) of the characters without disturbing the remaining characters' relative positions. (i.e., `"ACE"` is a subsequence of `"ABCDE"` while `"AEC"` is not).

The test cases are generated so that the answer fits on a 32-bit signed integer.

**Example 1:**

```Rust
Input: s = "rabbbit", t = "rabbit"
Output: 3
```

Explanation:
As shown below, there are 3 ways you can generate "rabbit" from S.
1. <u>**ra**</u>b<u>**bbit**</u>
2. <u>**rab**</u>b<u>**bit**</u>
3. <u>**rabb**</u>b<u>**it**</u>

**Example 2:**

```Rust
Input: s = "babgbag", t = "bag"
Output: 5
```

Explanation:
As shown below, there are 5 ways you can generate "bag" from S.
1. <u>**ba**</u>b<u>**g**</u>bag
2. <u>**ba**</u>bgba<u>**g**</u>
3. <u>**b**</u>abgb<u>**ag**</u>
4. ba<u>**b**</u>gb<u>**ag**</u>
5. babg<u>**bag**</u>

Constraints:

* `1 <= s.length, t.length <= 1000`
* `s` and `t` consist of English letters.
