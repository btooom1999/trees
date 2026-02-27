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

fn closest_value(root: Option<Rc<RefCell<TreeNode>>>, target: f32) -> i32 {
    let mut stack = Vec::new();
    let mut curr = root;
    let mut value = (-1, f32::MAX);
    while let Some(node_rc) = curr {
        let mut node = node_rc.borrow();
        let distance = (node.val as f32 - target).abs();
        if distance < value.1 {
            value = (node.val, distance);
        }

        if node.left.is_some() || node.right.is_some() {
            curr = if node.left.is_some() { node.left.clone() } else { node.right.clone() };
            if node.left.is_some() && node.right.is_some() { stack.push(node.right.clone()); }
        } else {
            curr = stack.pop().flatten();
        }
    }

    value.0
}

pub fn main() {
    let root = vec![Some(4), Some(2), Some(5), Some(1), Some(3)];
    let target = 3.714286_f32;
    // let root = vec![Some(1)];
    // let target = 4.428571;
    println!("{}", closest_value(vec_to_tree(root), target));
}
