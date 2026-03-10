use std::{cell::RefCell, rc::Rc};

use crate::count_univalue_subtrees;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, left: None, right: None }
    }
}

fn vec_to_tree(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(nums[0].unwrap())));
    let mut nodes = vec![root.clone()];

    let mut i = 1;
    while i < nums.len() {
        let mut new_nodes = Vec::new();
        for node_rc in nodes.iter() {
            let mut node = node_rc.borrow_mut();

            if let Some(&val) = nums.get(i) && let Some(val) = val {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(left.clone());
                new_nodes.push(left.clone());
            }
            i += 1;

            if let Some(&val) = nums.get(i) && let Some(val) = val {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.right = Some(right.clone());
                new_nodes.push(right.clone());
            }
            i += 1;
        }

        nodes = new_nodes;
    }

    Some(root)
}

fn helper(root: Option<Rc<RefCell<TreeNode>>>, count: &mut i32) -> bool {
    if let Some(node_rc) = root {
        let node = node_rc.borrow();

        let lv = helper(node.left.clone(), count);
        let rv = helper(node.right.clone(), count);

        if !lv
        || !rv
        || node.left.as_ref().is_some_and(|next| next.borrow().val != node.val)
        || node.right.as_ref().is_some_and(|next| next.borrow().val != node.val) {
            return false;
        }

        *count += 1;
        true
    } else {
        true
    }
}

fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut count = 0;
    helper(root, &mut count);

    count
}

pub fn main() {
    // let root = vec![Some(5), Some(1), Some(5), Some(5), Some(5), None, Some(5)];
    // let root = vec![Some(1), Some(1), Some(1), Some(5), Some(5), None, Some(5)];
    // let root = vec![Some(5), Some(1), Some(5), None, None, Some(5), Some(5)];
    let root = vec![Some(1), Some(1), Some(1), Some(5), Some(1)];
    println!("{}", count_unival_subtrees(vec_to_tree(root)));
}
