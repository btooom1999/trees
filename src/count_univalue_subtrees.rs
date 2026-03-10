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

fn count_unival_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if let Some(node_rc) = root {
        let node = node_rc.borrow();

        let lv = count_unival_subtrees(node.left.clone());
        let rv = count_unival_subtrees(node.right.clone());

        if node.left.as_ref().is_some_and(|left| left.borrow().val != node.val) {
            return lv + rv;
        }

        if node.right.as_ref().is_some_and(|right|right.borrow().val != node.val) {
            return lv + rv;
        }

        lv + rv + 1
    } else {
        0
    }
}

pub fn main() {
    // let root = vec![Some(5), Some(1), Some(5), Some(5), Some(5), None, Some(5)];
    let root = vec![Some(5), Some(4), Some(5), Some(4), Some(4), None, Some(5)];
    println!("{}", count_unival_subtrees(vec_to_tree(root)));
}
