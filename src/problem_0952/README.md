Problem 952. Largest Component Size by Common Factor (Hard)
===========================================================

See [on LeetCode](https://leetcode.com/problems/largest-component-size-by-common-factor/).

You are given an integer array of unique positive integers `nums`. Consider the following graph:

* There are `nums.length` nodes, labeled `nums[0]` to `nums[nums.length - 1]`,
* There is an undirected edge between `nums[i]` and `nums[j]` if `nums[i]` and `nums[j]` share a common factor greater than `1`.

Return the size of the largest connected component in the graph.

**Example 1:**

```Rust
Input: nums = [4,6,15,35]
Output: 4
```

**Example 2:**

```Rust
Input: nums = [20,50,9,63]
Output: 2
```

**Example 3:**

```Rust
Input: nums = [2,3,6,7,4,12,21,39]
Output: 8
```

**Constraints:**

* `1 <= nums.length <= 2 * 10⁴`
* `1 <= nums[i] <= 10⁵`
* All the values of `nums` are unique.
