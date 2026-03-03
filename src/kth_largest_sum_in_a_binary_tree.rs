use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

fn kth_largest_level_sum(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> i64 {
    let mut sum_list = Vec::new();
    let mut sum = 0i64;
    let mut nodes = VecDeque::from([root.clone(), None]);

    while let Some(front) = nodes.pop_front() {
        if let Some(node_rc) = front {
            let node = node_rc.borrow();
            sum += node.val as i64;

            if node.left.is_some() { nodes.push_back(node.left.clone()); }
            if node.right.is_some() { nodes.push_back(node.right.clone()); }
        } else {
            sum_list.push(sum);
            sum = 0;
            if !nodes.is_empty() { nodes.push_back(None); }
        }
    }

    sum_list.sort();
    *sum_list.get(sum_list.len() - k as usize).unwrap_or(&(-1))
}

pub fn main() {
    let root = vec![Some(5), Some(8), Some(9), Some(2), Some(1), Some(3), Some(7), Some(4), Some(6)];
    let k = 2;

    println!("{}", kth_largest_level_sum(vec_to_tree(root), k));
}
