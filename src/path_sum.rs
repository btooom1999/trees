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

fn vec_to_binary(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
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

            if let Some(val) = nums[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(left.clone());
                new_nodes.push(left);
            }
            i += 1;

            if i < nums.len() {
                if let Some(val) = nums[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.right = Some(right.clone());
                    new_nodes.push(right);
                }
                i += 1;
            }
        }

        nodes = new_nodes;
    }

    Some(root)
}

fn helper(node: Option<Rc<RefCell<TreeNode>>>, mut current_sum: i32, target_sum: i32) -> bool {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();
        current_sum += node.val;

        let lb = helper(node.left.clone(), current_sum, target_sum);
        let rb = helper(node.right.clone(), current_sum, target_sum);

        lb || rb || (current_sum == target_sum && node.left.is_none() && node.right.is_none())
    } else {
        current_sum == target_sum
    }
}

fn has_path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> bool {
    helper(root, 0, target_sum)
}

pub fn main() {
    // let root = vec![Some(5), Some(4), Some(8), Some(11), None, Some(13), Some(4), Some(7), Some(2), None, None, None, Some(1)];
    let root = vec![Some(1), Some(2)];
    let target_sum = 1;
    println!("{}", has_path_sum(vec_to_binary(root), target_sum));
}
