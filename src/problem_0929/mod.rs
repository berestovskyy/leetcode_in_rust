//
// Problem 929. Unique Email Addresses (Easy)
// See [on LeetCode](https://leetcode.com/problems/unique-email-addresses/).
//
// 0ms (100%)/2.3MB (88%)
// Space complexity: O(n)
// Runtime complexity: O(n)
//
// test problem_0929::tests::vec_1k                   ... bench:     165,898 ns/iter (+/- 9,317)
//

struct Solution {}

impl Solution {
    pub fn num_unique_emails(emails: Vec<String>) -> i32 {
        let mut set = std::collections::HashSet::with_capacity(emails.len());
        for email in emails {
            let mut new_email = String::with_capacity(email.len());
            let mut skipping_local = false;
            let mut parsing_local = true;
            for ch in email.chars() {
                match ch {
                    '@' => {
                        skipping_local = false;
                        parsing_local = false;
                        new_email.push(ch);
                    }
                    _ if skipping_local => {}
                    '.' if parsing_local => {}
                    '+' => skipping_local = true,
                    _ => new_email.push(ch),
                }
            }
            set.insert(new_email);
        }
        set.len() as i32
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        assert_eq!(
            Solution::num_unique_emails(vec![
                "test.email+alex@leetcode.com".into(),
                "test.e.mail+bob.cathy@leetcode.com".into(),
                "testemail+david@lee.tcode.com".into()
            ]),
            2
        );
        assert_eq!(
            Solution::num_unique_emails(vec![
                "a@leetcode.com".into(),
                "b@leetcode.com".into(),
                "c@leetcode.com".into(),
            ]),
            3
        );
    }

    #[bench]
    fn vec_1k(b: &mut Bencher) {
        let mut v = Vec::with_capacity(1_000);
        for i in 0..1_000 {
            v.push("a+b.c@gmail.com".into());
        }
        test::black_box(&v);
        b.iter(|| Solution::num_unique_emails(v.clone()));
    }
}
