//
// Problem 155. Min Stack (Easy)
// https://leetcode.com/problems/min-stack/
//
// 3ms/6.5MB
// Space complexity: O(n)
// Runtime complexity: O(1)
//
// test problem_0155::tests::min_stack_mix_1k          ... bench:       7,054 ns/iter (+/- 231)
// test problem_0155::tests::min_stack_new_1k          ... bench:       2,732 ns/iter (+/- 52)
// test problem_0155::tests::min_stack_push_1k         ... bench:       4,276 ns/iter (+/- 299)
// test problem_0155::tests::min_stack_push_get_min_1k ... bench:       4,100 ns/iter (+/- 238)
// test problem_0155::tests::min_stack_push_pop_1k     ... bench:       4,888 ns/iter (+/- 115)
// test problem_0155::tests::min_stack_push_top_1k     ... bench:       4,790 ns/iter (+/- 154)
//

struct MinStack {
    stack: Vec<i32>,
    mins: Vec<i32>,
}
impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            mins: Vec::new(),
        }
    }
    fn push(&mut self, val: i32) {
        self.stack.push(val);
        if val <= *self.mins.last().unwrap_or(&i32::MAX) {
            self.mins.push(val);
        }
    }
    fn pop(&mut self) {
        if self.top() == self.get_min() {
            self.mins.pop();
        }
        self.stack.pop();
    }
    fn top(&self) -> i32 {
        *self.stack.last().unwrap()
    }
    fn get_min(&self) -> i32 {
        *self.mins.last().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;

    #[test]
    fn example() {
        let mut s = MinStack::new();
        s.push(-2);
        s.push(0);
        s.push(-3);
        assert_eq!(s.get_min(), -3);
        s.pop();
        assert_eq!(s.top(), 0);
        assert_eq!(s.get_min(), -2);
    }

    #[bench]
    fn mix_1k(b: &mut Bencher) {
        let mut s = MinStack::new();
        b.iter(|| {
            for i in 0..1000 {
                s.push(i);
                assert_eq!(s.top(), i);
                assert_eq!(s.get_min(), 0);
            }
            for i in 0..1000 {
                assert_eq!(s.get_min(), 0);
                s.pop();
            }
            test::black_box(&s);
        });
    }
    #[bench]
    fn new_1k(b: &mut Bencher) {
        b.iter(|| {
            for i in 0..1000 {
                let mut s = MinStack::new();
                test::black_box(&s);
            }
        });
    }
    #[bench]
    fn new_push_1k(b: &mut Bencher) {
        b.iter(|| {
            let mut s = MinStack::new();
            for i in 0..1000 {
                s.push(i);
            }
            test::black_box(&s);
        });
    }
    #[bench]
    fn push_pop_1k(b: &mut Bencher) {
        b.iter(|| {
            let mut s = MinStack::new();
            for i in 0..1000 {
                s.push(i);
                test::black_box(&s);
                s.pop();
            }
        });
    }
    #[bench]
    fn push_top_1k(b: &mut Bencher) {
        b.iter(|| {
            let mut s = MinStack::new();
            for i in 0..1000 {
                s.push(i);
                let top = s.top();
                test::black_box(&top);
            }
        });
    }
    #[bench]
    fn push_get_min_1k(b: &mut Bencher) {
        b.iter(|| {
            let mut s = MinStack::new();
            for i in 0..1000 {
                s.push(i);
                let min = s.get_min();
                test::black_box(&min);
            }
        });
    }
}
