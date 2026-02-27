use std::{cell::RefCell, rc::Rc};

#[derive(PartialEq, Eq)]
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

            if i < nums.len() {
                if let Some(val) = nums[i] {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.left = Some(left.clone());
                    new_nodes.push(left.clone());
                }
                i += 1;
            }

            if i < nums.len() {
                if let Some(val) = nums[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.right = Some(right.clone());
                    new_nodes.push(right.clone());
                }
                i += 1;
            }
        }

        nodes = new_nodes;
    }

    Some(root)
}

fn is_subtree(root: Option<Rc<RefCell<TreeNode>>>, sub_root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let b = root == sub_root;
    if let Some(node_rc) = root {
        let node = node_rc.borrow();
        let lb = is_subtree(node.left.clone(), sub_root.clone());
        let rb = is_subtree(node.right.clone(), sub_root.clone());

        !(!lb && !rb && !b)
    } else {
        b
    }
}

pub fn main() {
    let root = vec![Some(3), Some(4), Some(5), Some(1), Some(2)];
    let sub_root = vec![Some(4), Some(1), Some(2)];
    println!("{}", is_subtree(vec_to_tree(root), vec_to_tree(sub_root)));
}
