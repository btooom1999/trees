use std::{cell::RefCell, rc::Rc};

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

fn leaf_similar(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut conditions = Vec::new();
    let mut stack = Vec::new();
    let mut curr = root1;

    while let Some(node_rc) = curr {
        let node = node_rc.borrow();

        if node.left.is_some() && node.right.is_some() {
            curr = node.left.clone();
            stack.push(node.right.clone());
        } else if node.left.is_some() || node.right.is_some() {
            curr = if node.left.is_some() { node.left.clone() } else { node.right.clone() };
        } else {
            conditions.push(node.val);
            curr = stack.pop().flatten();
        }
    }

    let mut curr = root2;
    while let Some(node_rc) = curr {
        let node = node_rc.borrow();

        if node.left.is_some() && node.right.is_some() {
            curr = node.right.clone();
            stack.push(node.left.clone());
        } else if node.left.is_some() || node.right.is_some() {
            curr = if node.right.is_some() { node.right.clone() } else { node.left.clone() };
        } else {
            if conditions.pop().is_none_or(|v| v != node.val) {
                return false;
            }

            curr = stack.pop().flatten();
        }
    }

    conditions.is_empty()
}

pub fn main() {
    let root1 = vec![Some(3), Some(5), Some(1), Some(6), Some(2), Some(9), Some(8), None, None, Some(7), Some(4)];
    let root2 = vec![Some(3), Some(5), Some(1), Some(6), Some(7), Some(4), Some(2), None, None, None, None, None, None, Some(9), Some(8)];
    println!("{}", leaf_similar(vec_to_tree(root1), vec_to_tree(root2)));
}
