use std::{cell::RefCell, clone, rc::Rc};

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
                    new_nodes.push(left);
                }
                i += 1;
            }

            if i < nums.len() {
                if let Some(val) = nums[i] {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.right = Some(right.clone());
                    new_nodes.push(right);
                }
                i += 1;
            }
        }
        nodes = new_nodes;
    }

    Some(root)
}

fn merge_trees(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut stack = Vec::new();
    let mut curr1 = root1.clone();
    let mut curr2 = root2.clone();

    while curr1.is_some() || curr2.is_some() {
        match (curr1, curr2) {
            (Some(node1_rc), Some(node2_rc)) => {
                let mut node1 = node1_rc.borrow_mut();
                let mut node2 = node2_rc.borrow_mut();

                let lb1 = node1.left.is_some();
                let rb1 = node1.right.is_some();
                let lb2 = node2.left.is_some();
                let rb2 = node2.right.is_some();

                node1.val += node2.val;

                // node1: left = Some, right = Some
                // node2: left = Some, right = Some
                if lb1 && rb1 && lb2 && rb2 {
                    curr1 = node1.left.clone();
                    curr2 = node2.left.clone();
                    stack.push((node1.right.clone(), node2.right.clone()));
                }
                // node1: left = Some
                // node2: left = Some
                else if lb1 && lb2 {
                    curr1 = node1.left.clone();
                    curr2 = node2.left.clone();
                    if rb2 { node1.right = node2.right.clone(); }
                }
                // node1: right = Some
                // node2: right = Some
                else if rb1 && rb2 {
                    curr1 = node1.right.clone();
                    curr2 = node2.right.clone();
                    if lb2 { node1.left = node2.left.clone(); }
                }
                // node1: left = Some
                // node2: right = Some
                // or
                // node1: right = Some
                // node2: left = Some
                else {
                    if lb2 { node1.left = node2.left.clone(); }
                    if rb2 { node1.right = node2.right.clone(); }
                    if let Some((node1, node2)) = stack.pop() {
                        curr1 = node1;
                        curr2 = node2;
                    } else {
                        curr1 = None;
                        curr2 = None;
                    }
                }
            }
            (_, Some(_)) => { return root2; }
            _ => { return root1; }
        }
    }

    root1
}

pub fn main() {
    let root1 = vec![Some(1), Some(3), Some(2), Some(5)];
    let root2 = vec![Some(2), Some(1), Some(3), None, Some(4), None, Some(7)];
    println!("{:?}", merge_trees(vec_to_tree(root1), vec_to_tree(root2)));
}
