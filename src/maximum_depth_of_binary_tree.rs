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

            // left side
            if i < nums.len() {
                if let Some(&val) = nums.get(i) && let Some(val) = val {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.left = Some(left.clone());
                    new_nodes.push(left.clone());
                }
                i += 1;
            }

            // right side
            if i < nums.len() {
                if let Some(&val) = nums.get(i) && let Some(val) = val {
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

fn max_depth(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut stack = Vec::new();
    let mut curr_depth = 1;
    let mut max_depth = 0;
    let mut curr = root;

    while let Some(node_rc) = curr {
        max_depth = max_depth.max(curr_depth);
        let node = node_rc.borrow();

        if node.left.is_some() {
            curr = node.left.clone();
            curr_depth += 1;
            if node.right.is_some() { stack.push((curr_depth, node.right.clone())); }
        } else if node.right.is_some() {
            curr = node.right.clone();
            curr_depth += 1;
        } else if let Some((depth, node)) = stack.pop() {
            curr_depth = depth;
            curr = node;
        } else {
            break;
        }
    }

    max_depth
}

pub fn main() {
    let root = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    println!("{}", max_depth(vec_to_tree(root)));
}
