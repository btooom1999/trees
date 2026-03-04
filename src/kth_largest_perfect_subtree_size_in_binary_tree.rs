use std::{cell::RefCell, rc::Rc};

use crate::kth_largest_perfect_subtree_size_in_binary_tree;

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

        nodes = new_nodes
    }

    Some(root)
}

fn helper(node: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<i32>) -> i32 {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        let lv = helper(node.left.clone(), vec);
        let rv = helper(node.right.clone(), vec);

        if lv != rv || lv == -1 || rv == -1 {
            return -1;
        }

        vec.push(lv + rv + 1);

        lv + rv + 1
    } else {
        0
    }
}

fn kth_largest_perfect_subtree(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i32 {
    let mut vec = Vec::new();
    helper(root, &mut vec);

    vec.sort();
    *vec.get(vec.len() - k as usize).unwrap_or(&-1)
}

pub fn main() {
    let root = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)];
    let k = 1;
    println!("{}", kth_largest_perfect_subtree(vec_to_tree(root), k));
}
