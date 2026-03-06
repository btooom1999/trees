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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, prev_str: String, smallest: &mut String) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        let c = (node.val as u8 + b'a') as char;
        let mut str = c.to_string();
        str.push_str(&prev_str);

        if node.left.is_none() && node.right.is_none() && str < *smallest {
            *smallest = str.clone();
        }

        helper(node.left.clone(), str.clone(), smallest);
        helper(node.right.clone(), str, smallest);
    }
}

fn smallest_from_leaf(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    let mut res = "|".to_string();
    helper(root, String::new(), &mut res);

    res
}

pub fn main() {
    // let root = vec![Some(0), Some(1), Some(2), Some(3), Some(4), Some(3), Some(4)];
    let root = vec![Some(4), Some(0), Some(1), Some(1)];
    println!("{}", smallest_from_leaf(vec_to_tree(root)));
}
