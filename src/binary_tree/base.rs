use std::rc::Rc;
use std::cell::RefCell;

type Link<T> = Option<Rc<RefCell<TreeNode<T>>>>;

pub struct TreeNode<T> {
    pub val: T,
    pub left: Link<T>,
    pub right: Link<T>
}

impl<T: Copy> TreeNode<T> {
    pub fn new(val: T) -> Self {
        TreeNode {
            val: val,
            left: None,
            right: None
        }
    }

    pub fn set_left(&mut self, val: T) {
        self.left = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }

    pub fn set_right(&mut self, val: T) {
        self.right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
    }

}

