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

        nodes = new_nodes
    }

    Some(root)
}

fn helper(
    node: Option<Rc<RefCell<TreeNode>>>,
    left_key: Option<i32>,
    right_key: Option<i32>,
    is_valid: &mut bool,
) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();
        if *is_valid && let Some(left_rc) = node.left.clone() {
            let left = left_rc.borrow();
            if left.val >= node.val
            || left_key.is_some_and(|key| left.val <= key)
            || right_key.is_some_and(|key| left.val >= key) {
                *is_valid = false;
                return;
            }

            let right_key = if let Some(key) = right_key {
                Some(key.min(node.val))
            } else {
                Some(node.val)
            };

            helper(node.left.clone(), left_key, right_key, is_valid);
        }

        if *is_valid && let Some(right_rc) = node.right.clone() {
            let right = right_rc.borrow();
            if right.val <= node.val
            || left_key.is_some_and(|key| right.val <= key)
            || right_key.is_some_and(|key| right.val >= key) {
                *is_valid = false;
                return;
            }

            let left_key = if let Some(key) = left_key {
                Some(key.max(node.val))
            } else {
                Some(node.val)
            };

            helper(node.right.clone(), left_key, right_key, is_valid);
        }
    }
}

fn is_valid_bst(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let mut res = true;
    helper(root, None, None, &mut res);

    res
}

pub fn main() {
    let root = vec![Some(5), Some(3), Some(8), Some(0), Some(4), Some(4), Some(9), Some(-1), Some(2)];
    println!("{}", is_valid_bst(vec_to_tree(root)));
}
