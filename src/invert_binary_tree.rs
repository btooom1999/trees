use std::{cell::RefCell, rc::Rc};

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

            // left side
            if i < nums.len() {
                if let Some(&val) = nums.get(i) && let Some(val) = val {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.left = Some(left.clone());
                    new_nodes.push(left.clone());
                }

                i += 1;
            }

            // right side
            if i < nums.len() {
                if let Some(&val) = nums.get(i) && let Some(val) = val {
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

fn invert_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut stack = Vec::new();
    let mut curr = root.clone();

    while let Some(node_rc) = curr {
        let mut node = node_rc.borrow_mut();
        if node.left.is_some() || node.right.is_some() {
            (node.left, node.right) = (node.right.take(), node.left.take());

            if node.left.is_some() {
                curr = node.left.clone();
                if node.right.is_some() { stack.push(node.right.clone()) };
            } else {
                curr = node.right.clone();
            }
        } else {
            curr = stack.pop().flatten();
        }
    }

    root
}

pub fn main() {
    let root = [Some(4), Some(2), Some(7), Some(1), Some(3), Some(6), Some(9)];
    println!("{:#?}", invert_tree(vec_to_tree(root.into())));
}
