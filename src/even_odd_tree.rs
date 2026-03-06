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

fn is_even_odd_tree(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut nodes = VecDeque::from([root.clone()]);
    if root.is_some() {
        nodes.push_back(None);
    }

    let mut increasing = true;
    let mut val = 0;
    while let Some(front) = nodes.pop_front() {
        if let Some(node_rc) = front {
            let node = node_rc.borrow();
            if increasing {
                if node.val % 2 == 0 { return false };
                if val >= node.val { return false };
            } else {
                if node.val % 2 != 0 { return false };
                if val <= node.val { return false };
            }
            if node.left.is_some() { nodes.push_back(node.left.clone()); }
            if node.right.is_some() { nodes.push_back(node.right.clone()); }
            val = node.val;
        } else {
            increasing = !increasing;
            if increasing { val = 0; } else { val = 1_000_001; }
            if !nodes.is_empty() { nodes.push_back(None); }
        }
    }

    true
}

pub fn main() {
    let root = vec![Some(1), Some(10), Some(4), Some(3), None, Some(7), Some(9), Some(12), Some(8), Some(6), None, None, Some(2)];
    println!("{}", is_even_odd_tree(vec_to_tree(root)));
}
