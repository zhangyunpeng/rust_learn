
use std::{collections::VecDeque, rc::Rc, cell::RefCell};

use super::base::TreeNode;

pub fn run(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut stack = VecDeque::new();
    if let Some(node) = root {
        stack.push_back(node);
    }
    while !stack.is_empty() {
        let length = stack.len();
        let mut tmp = Vec::new();
        for _ in 0..length {
            let node = stack.pop_front().unwrap();
            tmp.push(node.borrow().val);
            if node.borrow().left.is_some() {
                stack.push_back(node.borrow().left.clone().unwrap())
            }
            if node.borrow().right.is_some() {
                stack.push_back(node.borrow().right.clone().unwrap())
            }
        }
        result.push(tmp);
    }
    result
}