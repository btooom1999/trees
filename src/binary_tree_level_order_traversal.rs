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

        nodes = new_nodes
    }

    Some(root)
}

fn level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut nodes = vec![root.clone()];

    loop {
        if nodes.is_empty() {
            return res;
        }

        let mut new_res = Vec::new();
        let mut new_nodes = Vec::new();
        for node_rc in nodes.iter().flatten() {
            let node = node_rc.borrow();
            new_nodes.push(node.left.clone());
            new_nodes.push(node.right.clone());
            new_res.push(node.val);
        }

        if !new_res.is_empty() { res.push(new_res); }
        nodes = new_nodes;
    }

    res
}

pub fn main() {
    let root = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    println!("{:?}", level_order(vec_to_tree(root)));
}
