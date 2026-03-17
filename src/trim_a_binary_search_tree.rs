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

fn helper(root: &mut Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) {
    while let Some(node_rc) = root.clone() {
        let mut node = node_rc.borrow_mut();
        if node.val < low {
            *root = node.right.take();
        } else if node.val > high {
            *root = node.left.take();
        } else {
            break;
        }
    }

    if let Some(node_rc) = root {
        let mut node = node_rc.borrow_mut();
        helper(&mut node.left, low, high);
        helper(&mut node.right, low, high);
    }
}

fn trim_bst(mut root: Option<Rc<RefCell<TreeNode>>>, low: i32, high: i32) -> Option<Rc<RefCell<TreeNode>>> {
    helper(&mut root, low, high);

    root
}

pub fn main() {
    let root = [Some(1), Some(0), Some(200)];
    let low = 2;
    let high = 2;
    println!("{:?}", trim_bst(vec_to_tree(root.into()), low, high));
}
