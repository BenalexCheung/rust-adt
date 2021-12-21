struct MinStack {
    stack: Vec<i32>,
    min_stack: Vec<i32>,
}

impl MinStack {
    fn new() -> Self {
        MinStack {
            stack: Vec::new(),
            min_stack: Vec::new(),
        }
    }

    fn push(&mut self, x: i32) {
        self.stack.push(x);
        if self.min_stack.is_empty() || x <= *self.min_stack.last().unwrap() {
            self.min_stack.push(x);
        }
    }

    fn pop(&mut self) {
        if self.stack.is_empty() {
            return;
        }
        if self.stack.pop().unwrap() == *self.min_stack.last().unwrap() {
            self.min_stack.pop();
        }
    }

    fn top(&self) -> i32 {
        return *self.stack.last().unwrap();
    }

    fn get_min(&self) -> i32 {
        return *self.min_stack.last().unwrap();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let mut min_stack = MinStack::new();
        min_stack.push(-2);
        min_stack.push(0);
        min_stack.push(-3);
        println!("{}",min_stack.get_min());
        min_stack.pop();
        println!("{}",min_stack.top());
        println!("{}",min_stack.get_min());
        assert_eq!(2 + 2, 4);
    }
}
