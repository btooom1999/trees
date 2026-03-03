use std::{cell::RefCell, rc::{self, Rc}};

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

            if let Some(&val) = nums.get(i) && let Some(val) = val {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(left.clone());
                new_nodes.push(left.clone())
            }
            i += 1;

            if let Some(&val) = nums.get(i) && let Some(val) = val {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.right = Some(right.clone());
                new_nodes.push(right.clone())
            }
            i += 1;
        }

        nodes = new_nodes
    }

    Some(root)
}

fn delete_node(root: Option<Rc<RefCell<TreeNode>>>, key: i32) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node_rc) = root {
        let mut node = node_rc.borrow_mut();
        if node.val < key {
            node.right = delete_node(node.right.clone(), key);
        } else if node.val > key {
            node.left = delete_node(node.left.clone(), key);
        } else {
            if node.left.is_some() && node.right.is_some() {
                let mut curr = node.right.clone();
                while curr.as_ref().unwrap().borrow().left.is_some() {
                    curr = curr.unwrap().borrow().left.clone();
                }

                node.val = curr.unwrap().borrow().val;
                node.right = delete_node(node.right.clone(), node.val);
                return Some(node_rc.clone())
            }

            return if node.left.is_some() { node.left.clone() } else { node.right.clone() };
        }

        Some(node_rc.clone())
    } else {
        None
    }
}

pub fn main() {
    let root = vec![Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)];
    let key = 5;
    println!("{:?}", delete_node(vec_to_tree(root), key))
}
