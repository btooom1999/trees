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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> (i32, i32) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        let left = helper(node.left.clone(), max);
        let right = helper(node.right.clone(), max);

        let mut depth = 0;
        if node.val == left.0 && node.val == right.0 {
            *max = std::cmp::max(*max, left.1 + right.1 + 2);
            depth = std::cmp::max(left.1, right.1) + 1;
        } else if node.val == left.0 {
            depth = left.1 + 1;
        } else if node.val == right.0 {
            depth = right.1 + 1;
        }

        *max = std::cmp::max(*max, depth);

        (node.val, depth)
    } else {
        (-1001, 0)
    }
}

fn longest_univalue_path(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max = 0;
    helper(root, &mut max);

    max
}

pub fn main() {
    let root = [Some(5), Some(4), Some(5), Some(1), Some(1), None, Some(5)];
    println!("{}", longest_univalue_path(vec_to_tree(root.into())));
}
