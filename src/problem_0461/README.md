Problem 461. Hamming Distance (Easy)
====================================

See [on LeetCode](https://leetcode.com/problems/hamming-distance/).

The Hamming distance between two integers is the number of positions at which the corresponding bits are different.

Given two integers `x` and `y`, return the Hamming distance between them.

**Example 1:**

```Rust
Input: x = 1, y = 4
Output: 2
```

Explanation:

```Rust
1   (0 0 0 1)
4   (0 1 0 0)
       ↑   ↑
```

The above arrows point to positions where the corresponding bits are different.

**Example 2:**

```Rust
Input: x = 3, y = 1
Output: 1
```

**Constraints:**

* `0 <= x, y <= 2³¹ - 1`
