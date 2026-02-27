use std::{cell::RefCell, rc::Rc};

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, left: None ,right: None }
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

                i += 1
            }

            if i < nums.len() {
                if let Some(val) = nums[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.left = Some(right.clone());
                    new_nodes.push(right);
                }

                i += 1
            }
        }

        nodes = new_nodes;
    }

    Some(root)
}

fn helper(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, bool) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();
        let (lh, lb) = helper(node.left.clone());
        let (rh, rb) = helper(node.right.clone());

        let balanced = lb && rb && (lh - rh).abs() <= 1;

        (1 + std::cmp::max(lh, rh), balanced)
    } else {
        (0, true)
    }
}

fn is_balanced(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    helper(root).1
}

pub fn main() {
    let root = vec![Some(1), Some(2), Some(2), Some(3), Some(3), None, None, Some(4), Some(4)];
    println!("{}", is_balanced(vec_to_tree(root)));
}
