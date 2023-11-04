use std::{collections::VecDeque, rc::Rc, cell::RefCell};
use super::base::TreeNode;

pub fn run(root: &Option<Rc<RefCell<TreeNode<i32>>>>) -> Vec<i32> {
    fn helper(node : &Option<Rc<RefCell<TreeNode<i32>>>>, list: &mut Vec<i32>) {
        if let Some(node) = node {
            let act_node = node.borrow();
            list.push(act_node.val);
            helper(&act_node.left, list);
            helper(&act_node.right, list);
        }
    }
    let mut result = Vec::new();
    helper(root, &mut result);
    return result;
}

pub fn run_no_rec(root: Option<Rc<RefCell<TreeNode<i32>>>>) -> Vec<i32> {
    let mut result = Vec::new();
    let mut stack = VecDeque::new();
    if let Some(node) = root {
        stack.push_back(node);
    }
    while !stack.is_empty() {
        let node = stack.pop_back().unwrap();
        result.push(node.borrow().val);
        if node.borrow().right.is_some() {
            stack.push_back(node.borrow().right.clone().unwrap());
        }
        if node.borrow().left.is_some() {
            stack.push_back(node.borrow().left.clone().unwrap());
        }
    }

    result
}