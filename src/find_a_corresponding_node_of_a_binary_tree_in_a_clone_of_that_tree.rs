use std::{cell::RefCell, rc::{self, Rc}};

#[derive(Debug, PartialEq, Eq)]
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

fn get_target_copy(
    original: Option<Rc<RefCell<TreeNode>>>,
    cloned: Option<Rc<RefCell<TreeNode>>>,
    target: Option<Rc<RefCell<TreeNode>>>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node_rc) = cloned {
        let mut node = node_rc.borrow_mut();

        if Some(node_rc.clone()) == target {
            return Some(node_rc.clone());
        }

        if node.left == target {
            return node.left.take();
        }
        if node.right == target {
            return node.right.take();
        }

        let left = get_target_copy(original.clone(), node.left.clone(), target.clone());
        let right = get_target_copy(original.clone(), node.right.clone(), target.clone());

        if left.is_none() { right } else { left }
    } else {
        None
    }
}

pub fn main() {
    let tree = vec![Some(7), Some(4), Some(3), None, None, Some(6), Some(19)];
    let target = vec![Some(3), Some(6), Some(19)];
    println!("{:?}", get_target_copy(vec_to_tree(tree.clone()), vec_to_tree(tree), vec_to_tree(target)))
}
