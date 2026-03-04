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

fn zigzag_level_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    let mut row = 1;
    let mut nodes = VecDeque::from([root.clone()]);
    if root.is_some() {
        nodes.push_back(None);
    }

    let mut res = Vec::new();
    let mut data = VecDeque::new();
    while let Some(front) = nodes.pop_front() {
        if let Some(node_rc) = front {
            let node = node_rc.borrow();
            if row % 2 == 0 {
                data.push_front(node.val);
            } else {
                data.push_back(node.val);
            }
            if node.left.is_some() { nodes.push_back(node.left.clone()); }
            if node.right.is_some() { nodes.push_back(node.right.clone()); }
        } else {
            if !data.is_empty() { res.push(data.clone().into()); }
            data.clear();
            row += 1;
            if !nodes.is_empty() { nodes.push_back(None); }
        }
    }

    res
}

pub fn main() {
    let root = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    println!("{:?}", zigzag_level_order(vec_to_tree(root)));
}
