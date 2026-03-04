use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

fn max_level_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut nodes = VecDeque::from([root.clone(), None]);
    let mut depth = 1;
    let mut max = (depth, i32::MIN);

    let mut sum = 0;
    while let Some(front) = nodes.pop_front() {
        if let Some(node_rc) = front {
            let node = node_rc.borrow();
            sum += node.val;
            if node.left.is_some() { nodes.push_back(node.left.clone()); }
            if node.right.is_some() { nodes.push_back(node.right.clone()); }
        } else {
            if sum > max.1 {
                max = (depth, sum);
            }
            sum = 0;
            depth += 1;
            if !nodes.is_empty() { nodes.push_back(None); }
        }
    }

    max.0
}

pub fn main() {
    let root = vec![Some(1), Some(7), Some(0), Some(7), Some(-8), None, None];
    println!("{:?}", max_level_sum(vec_to_tree(root)));
}
