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