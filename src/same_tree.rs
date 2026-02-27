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

fn is_same_tree(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (p, q) {
        (Some(p_rc), Some(q_rc)) => {
            let p = p_rc.borrow();
            let q = q_rc.borrow();

            let lb = is_same_tree(p.left.clone(), q.left.clone());
            let rb = is_same_tree(p.right.clone(), q.right.clone());

            lb && rb && p.val == q.val
        }
        (None, Some(_)) | (Some(_), None) => {
            false
        }
        _ => {
            true
        }
    }
}

pub fn main() {
    let p = [Some(1), Some(2), Some(3)];
    let q = [Some(1), Some(2), Some(3)];

    println!("{}", is_same_tree(vec_to_tree(p.into()), vec_to_tree(q.into())));
}
