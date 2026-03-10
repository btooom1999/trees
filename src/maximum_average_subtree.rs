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
                node.right= Some(right.clone());
                new_nodes.push(right.clone());
            }
            i += 1;
        }

        nodes = new_nodes;
    }

    Some(root)
}

fn helper(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, f64) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        let left = helper(node.left.clone());
        let right = helper(node.right.clone());

        let average = left.2.max(right.2);
        let sum = node.val + left.0 + right.0;
        let n = 1 + left.1 + right.1;


        (sum, n, average.max(sum as f64 / n as f64))
    } else {
        (0, 0, 0_f64)
    }
}

fn maximum_average_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> f64 {
    helper(root).2
}

pub fn main() {
    let root = vec![Some(5), Some(3), Some(8)];
    println!("{}", maximum_average_subtree(vec_to_tree(root)));
}
