//
// Problem 721. Accounts Merge (Medium)
// See [on LeetCode](https://leetcode.com/problems/accounts-merge/).
//
// 36ms (46%)/4.3MB (47%)
// Space complexity: O(n)
// Runtime complexity: O(n²)
//
// test problem_0721::tests::vec_1k                   ... bench:   1,041,468 ns/iter (+/- 147,772)
//

struct Solution {}

use std::{collections::HashMap, slice::SliceIndex};

// Based on:
// https://leetcode.com/problems/accounts-merge/discuss/1601905/Rust-Solution-or-UnionFind-or-Simple-55-Lines
impl Solution {
    pub fn accounts_merge(accounts: Vec<Vec<String>>) -> Vec<Vec<String>> {
        pub fn find<'a>(fa: &mut HashMap<&'a str, &'a str>, x: &'a str) -> &'a str {
            if x == fa[x] {
                return x;
            }
            let fd = find(fa, fa[x]);
            fa.insert(x, fd);
            fa[x]
        }

        pub fn union<'a>(fa: &mut HashMap<&'a str, &'a str>, x: &'a str, y: &'a str) {
            let a = find(fa, x);
            let b = find(fa, y);
            fa.insert(b, a);
        }

        let mut fa = HashMap::new();
        let mut email_name = HashMap::new();
        for acc in &accounts {
            let name = &acc[0];
            for a in acc.iter().skip(1) {
                fa.insert(a as &str, a as &str);
                email_name.entry(a as &str).or_insert(name);
            }
        }
        for acc in &accounts {
            for i in 2..acc.len() {
                union(&mut fa, &acc[1], &acc[i]);
            }
        }
        let mut name_emails = HashMap::new();
        for em in email_name.keys() {
            name_emails
                .entry(find(&mut fa, *em))
                .or_insert_with(Vec::new)
                .push(*em);
        }
        for v in name_emails.values_mut() {
            v.sort_unstable();
        }

        let mut res = Vec::new();
        for fa_email in name_emails.keys() {
            let mut data = vec![email_name[*fa_email].to_string()];
            for t in name_emails.get(fa_email).unwrap().iter() {
                data.push(t.to_string());
            }
            res.push(data);
        }
        res
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        {
            let mut res = Solution::accounts_merge(vec![
                vec![
                    "John".into(),
                    "johnsmith@mail.com".into(),
                    "john_newyork@mail.com".into(),
                ],
                vec![
                    "John".into(),
                    "johnsmith@mail.com".into(),
                    "john00@mail.com".into(),
                ],
                vec!["Mary".into(), "mary@mail.com".into()],
                vec!["John".into(), "johnnybravo@mail.com".into()],
            ]);
            res.sort_unstable();
            assert_eq!(
                res,
                vec![
                    vec![
                        String::from("John"),
                        String::from("john00@mail.com"),
                        String::from("john_newyork@mail.com"),
                        String::from("johnsmith@mail.com"),
                    ],
                    vec![String::from("John"), String::from("johnnybravo@mail.com")],
                    vec![String::from("Mary"), String::from("mary@mail.com")],
                ]
            );
        }
        {
            let mut res = Solution::accounts_merge(vec![
                vec![
                    "Gabe".into(),
                    "Gabe0@m.co".into(),
                    "Gabe3@m.co".into(),
                    "Gabe1@m.co".into(),
                ],
                vec![
                    "Kevin".into(),
                    "Kevin3@m.co".into(),
                    "Kevin5@m.co".into(),
                    "Kevin0@m.co".into(),
                ],
                vec![
                    "Ethan".into(),
                    "Ethan5@m.co".into(),
                    "Ethan4@m.co".into(),
                    "Ethan0@m.co".into(),
                ],
                vec![
                    "Hanzo".into(),
                    "Hanzo3@m.co".into(),
                    "Hanzo1@m.co".into(),
                    "Hanzo0@m.co".into(),
                ],
                vec![
                    "Fern".into(),
                    "Fern5@m.co".into(),
                    "Fern1@m.co".into(),
                    "Fern0@m.co".into(),
                ],
            ]);
            res.sort_unstable();
            assert_eq!(
                res,
                vec![
                    vec![
                        String::from("Ethan"),
                        String::from("Ethan0@m.co"),
                        String::from("Ethan4@m.co"),
                        String::from("Ethan5@m.co"),
                    ],
                    vec![
                        String::from("Fern"),
                        String::from("Fern0@m.co"),
                        String::from("Fern1@m.co"),
                        String::from("Fern5@m.co"),
                    ],
                    vec![
                        String::from("Gabe"),
                        String::from("Gabe0@m.co"),
                        String::from("Gabe1@m.co"),
                        String::from("Gabe3@m.co"),
                    ],
                    vec![
                        String::from("Hanzo"),
                        String::from("Hanzo0@m.co"),
                        String::from("Hanzo1@m.co"),
                        String::from("Hanzo3@m.co"),
                    ],
                    vec![
                        String::from("Kevin"),
                        String::from("Kevin0@m.co"),
                        String::from("Kevin3@m.co"),
                        String::from("Kevin5@m.co"),
                    ],
                ]
            );
        }
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push(vec![i.to_string(), i.to_string(), (i % 3).to_string()]);
        }
        test::black_box(&v);
        b.iter(|| Solution::accounts_merge(v.clone()));
    }
}
