Problem 899. Orderly Queue (Hard)
=================================

See [on LeetCode](https://leetcode.com/problems/orderly-queue/).

You are given a string `s` and an integer `k`. You can choose one of the first `k` letters of `s` and append it at the end of the string.

Return the lexicographically smallest string you could have after applying the mentioned step any number of moves.

**Example 1:**

```Rust
Input: s = "cba", k = 1
Output: "acb"
```

Explanation:

1. In the first move, we move the 1st character `'c'` to the end, obtaining the string `"bac"`.
2. In the second move, we move the 1st character `'b'` to the end, obtaining the final result `"acb"`.

**Example 2:**

```Rust
Input: s = "baaca", k = 3
Output: "aaabc"
```

Explanation:

1. In the first move, we move the 1st character `'b'` to the end, obtaining the string `"aacab"`.
2. In the second move, we move the 3rd character `'c'` to the end, obtaining the final result `"aaabc"`.

**Constraints:**

1. `1 <= k <= s.length <= 1000`
2. `s` consist of lowercase English letters.
