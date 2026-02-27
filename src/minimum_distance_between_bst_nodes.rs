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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, prev_num: &mut Option<i32>, diff: &mut i32) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        helper(node.left.clone(), prev_num, diff);

        if let Some(inner_value) = prev_num {
            *diff = std::cmp::min(*diff, node.val - *inner_value);
        }

        *prev_num = Some(node.val);

        helper(node.right.clone(), prev_num, diff);
    }
}

fn min_diff_in_bst(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut diff = i32::MAX;
    helper(root, &mut None, &mut diff);

    diff
}

pub fn main() {
    let root = vec![Some(4), Some(2), Some(6), Some(1), Some(3)];
    println!("{:?}", min_diff_in_bst(vec_to_tree(root)));
}

