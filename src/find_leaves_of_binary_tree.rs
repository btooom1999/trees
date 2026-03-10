use std::{cell::RefCell, collections::HashMap, rc::Rc};

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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, hashmap: &mut Vec<Vec<i32>>) -> usize {
    if let Some(node_rc) = node {
        let mut node = node_rc.borrow();

        let left = helper(node.left.clone(), hashmap);
        let right = helper(node.right.clone(), hashmap);

        let i = left.max(right);
        if let Some(values) = hashmap.get_mut(i) {
            values.push(node.val);
        } else {
            hashmap.push(vec![node.val]);
        }

        i+1
    } else {
        0
    }
}

fn find_leaves(mut root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut hashmap = Vec::new();
    helper(root, &mut hashmap);

    hashmap
}

pub fn main() {
    let root = vec![Some(1), Some(2), Some(3), Some(4), Some(5)];
    println!("{:?}", find_leaves(vec_to_tree(root)));
}
