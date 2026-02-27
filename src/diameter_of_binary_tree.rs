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
                i += 1;
            }

            if i < nums.len() {
                if let Some(val) = nums[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.right = Some(right.clone());
                    new_nodes.push(right.clone());
                }
                i += 1;
            }
        }
        nodes = new_nodes;
    }

    Some(root)
}

fn helper(node: Option<Rc<RefCell<TreeNode>>>, max_diameter: &mut i32) -> i32 {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        let l_height = helper(node.left.clone(), max_diameter);
        let r_height = helper(node.right.clone(), max_diameter);

        *max_diameter = std::cmp::max(*max_diameter, l_height + r_height);

        1 + std::cmp::max(l_height, r_height)
    } else {
        0
    }
}

fn diameter_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max = 0;
    helper(root, &mut max);

    max
}

pub fn main() {
    let root = [Some(1), Some(2), Some(3), Some(4), Some(5)];
    println!("{}", diameter_of_binary_tree(vec_to_tree(root.into())));
}
