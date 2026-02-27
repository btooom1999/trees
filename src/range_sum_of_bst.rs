use std::{cell::RefCell, rc::Rc};

use crate::range_sum_of_bst;

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, left: None, right: None }
    }
}

fn vec_to_nums(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
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

fn range_sum_bst(root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> i32 {
    if let Some(node_rc) = root {
        let node = node_rc.borrow();
        let ls = range_sum_bst(node.left.clone(), low, high);
        let rs = range_sum_bst(node.right.clone(), low, high);
        let bonus = if node.val < low || node.val > high { 0 } else { node.val };

        ls + rs + bonus
    } else {
        0
    }
}

pub fn main() {
    let root = vec![Some(10), Some(5), Some(15), Some(3), Some(7), Some(13), Some(18), Some(1), None, Some(6)];
    let low = 6;
    let high = 10;
    println!("{}", range_sum_bst(vec_to_nums(root), low, high))
}
