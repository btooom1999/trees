use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

fn right_side_view(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut nodes = VecDeque::from([root.clone()]);

    loop {
        if nodes.is_empty() {
            return res;
        }

        let mut new_nodes = VecDeque::new();
        let mut val = -1;
        while let Some(node) = nodes.pop_front() {
            if let Some(node_rc) = node {
                let node = node_rc.borrow();
                val = node.val;
                new_nodes.push_back(node.left.clone());
                new_nodes.push_back(node.right.clone());
            }
        }

        if val != -1 { res.push(val) };
        nodes = new_nodes;
    }
}

pub fn main() {
    let root = vec![Some(1), Some(2), Some(3), Some(4), None, None, None, Some(5)];
    println!("{:?}", right_side_view(vec_to_tree(root)));
}
