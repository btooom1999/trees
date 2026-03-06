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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, mut vec: Vec<i32>) -> i32 {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();
        vec[node.val as usize] = (vec[node.val as usize] + 1) % 2;

        if node.left.is_none() && node.right.is_none() {
            if vec.iter().fold(0, |acc, &num| acc + if num > 0 {1} else {0}) >= 2 {
                return 0;
            } else {
                return 1;
            }
        }

        helper(node.left.clone(), vec.clone()) + helper(node.right.clone(), vec.clone())
    } else {
        0
    }
}

fn pseudo_palindromic_paths (root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    helper(root, vec![0;10])
}

pub fn main() {
    let root = vec![Some(2), Some(3), Some(1), Some(3), Some(1), None, Some(1)];
    println!("{:?}", pseudo_palindromic_paths(vec_to_tree(root)));
}
