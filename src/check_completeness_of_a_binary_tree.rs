use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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
                let left = Rc::new(RefCell::new(TreeNode::new(nums[0].unwrap())));
                node.left = Some(left.clone());
                new_nodes.push(left.clone());
            }
            i += 1;

            if let Some(&val) = nums.get(i) && let Some(val) = val {
                let right = Rc::new(RefCell::new(TreeNode::new(nums[0].unwrap())));
                node.right = Some(right.clone());
                new_nodes.push(right.clone());
            }
            i += 1;
        }

        nodes = new_nodes;
    }

    Some(root)
}

fn is_complete_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut nodes = vec![root.clone()];
    let mut row = 1;
    let mut has_none = false;
    let mut last = false;

    loop {
        let mut new_nodes = Vec::new();
        for node_rc in nodes.iter().flatten() {
            let node = node_rc.borrow();
            if node.left.is_none() {
                has_none = true;
            } else if has_none {
                return false;
            } else {
                new_nodes.push(node.left.clone());
            }

            if node.right.is_none() {
                has_none = true;
            } else if has_none {
                return false;
            } else {
                new_nodes.push(node.right.clone());
            }
        }

        if new_nodes.is_empty() {
            return true;
        } else if new_nodes.len() as i32 != 2i32.pow(row) {
            if last { return false; }
            last = true;
        }

        row += 1;
        nodes = new_nodes;
    }
}

pub fn main() {
    let root = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6)];
    println!("{}", is_complete_tree(vec_to_tree(root.clone())));
}
