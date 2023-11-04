use std::cell::RefCell;
use std::rc::Rc;
use rust_learn::binary_tree::base::TreeNode;
use rust_learn::binary_tree::dfs;

#[test]
fn dfs() {
    let tree = TreeNode{
        val: 1,
        left: Some(Rc::new(RefCell::new(TreeNode{
            val: 2,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 4,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 5,
                left: None,
                right: None,
            }))),
        }))),
        right: Some(Rc::new(RefCell::new(TreeNode{
            val: 3,
            left: Some(Rc::new(RefCell::new(TreeNode{
                val: 6,
                left: None,
                right: None,
            }))),
            right: Some(Rc::new(RefCell::new(TreeNode{
                val: 7,
                left: None,
                right: None,
            }))),
        }))),
    };

    let res = dfs::run(Some(Rc::new(RefCell::new(tree))));
    assert_eq!(res, vec![
        vec![1],
        vec![2,3],
        vec![4,5,6,7]
    ]);
}