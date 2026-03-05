use std::{cell::RefCell, rc::Rc};

use crate::lowest_common_ancestor_of_a_binary_tree;

#[derive(Debug)]
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

fn width_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut nodes = vec![(root.clone(), 1)];
    let mut width = 0;
    let mut max_width = 0;

    loop {
        let mut new_nodes = Vec::new();
        width = 0;
        for i in 0..nodes.len() {
            let (node_rc, curr_width) = (nodes[i].0.clone(), nodes[i].1);
            if let Some(node_rc) = node_rc {
                let node = node_rc.borrow();
                max_width = max_width.max(curr_width);

                if width > 0 && i > 0 {
                    width += (curr_width-nodes[i-1].1-1)*2;
                }

                if node.left.is_some() {
                    width += 1;
                    new_nodes.push((node.left.clone(), width));
                } else if width > 0 {
                    width += 1;
                }

                if node.right.is_some() {
                    width += 1;
                    new_nodes.push((node.right.clone(), width));
                } else if width > 0 {
                    width += 1;
                }
            }
        }

        if new_nodes.is_empty() {
            return max_width;
        }

        nodes = new_nodes;
    }
}

pub fn main() {
    let root = vec![Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)];
    println!("{}", width_of_binary_tree(vec_to_tree(root)));
}
