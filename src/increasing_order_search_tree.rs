use std::{cell::RefCell, rc::Rc};

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

fn helper(node: Option<Rc<RefCell<TreeNode>>>) -> (Option<Rc<RefCell<TreeNode>>>, Option<Rc<RefCell<TreeNode>>>) {
    if let Some(node_rc) = node {
        let mut node = node_rc.borrow_mut();
        let left = helper(node.left.take());
        let right = helper(node.right.take());

        if left.0.is_some() && right.0.is_some() {
            left.1.unwrap().borrow_mut().right = Some(node_rc.clone());
            node.right = right.0;
            return (left.0, right.1);
        }

        if left.0.is_some() {
            left.1.unwrap().borrow_mut().right = Some(node_rc.clone());
            return (left.0, Some(node_rc.clone()));
        }

        if right.0.is_some() {
            node.right = right.0;
            return (Some(node_rc.clone()), right.1);
        }

        (Some(node_rc.clone()), Some(node_rc.clone()))
    } else {
        (None, None)
    }
}

fn increasing_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    helper(root).0
}

pub fn main() {
    let root = [Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(8), Some(1), None, None, None,Some(7),Some(9)];
    println!("{:?}", increasing_bst(vec_to_tree(root.into())));
}
