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
    let n = nums.len();
    while i < n {
        let mut new_nodes = Vec::new();
        for node_rc in nodes.iter() {
            let mut node = node_rc.borrow_mut();

            if i < n && let Some(val) = nums[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(left.clone());
                new_nodes.push(left.clone());
            }
            i += 1;

            if i < n && let Some(val) = nums[i] {
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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> Option<i32> {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        let left = helper(node.left.clone(), max);
        let right = helper(node.right.clone(), max);
        let mut current = node.val;
        match (left, right) {
            (Some(left), Some(right)) => {
                *max = std::cmp::max(*max, node.val + left + right);
                current = current.max(node.val + left);
                current = current.max(node.val + right);
            }
            (Some(val), _) | (_, Some(val)) => {
                current = current.max(node.val + val);
            }
            _ => {}
        }

        *max = std::cmp::max(*max, current);

        Some(current)
    } else {
        None
    }
}

fn max_path_sum(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max = i32::MIN;
    helper(root, &mut max);

    max
}

pub fn main() {
    let root = vec![Some(-10), Some(9), Some(20), None, None, Some(15), Some(7)];
    println!("{}", max_path_sum(vec_to_tree(root)));
}
