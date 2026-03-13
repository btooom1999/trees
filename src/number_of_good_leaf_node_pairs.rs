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
    let n = nums.len();
    while i < nums.len() {
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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, distance: i32, count: &mut i32) -> Vec<i32> {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        if node.left.is_none() && node.right.is_none() {
            return vec![0];
        }

        let left = helper(node.left.clone(), distance, count);
        let right = helper(node.right.clone(), distance, count);

        for ln in &left {
            for rn in &right {
                if ln + rn + 1 < distance {
                    *count += 1;
                }
            }
        }

        left.into_iter().chain(right).map(|d| d + 1).filter(|v| *v < distance).collect()
    } else {
        vec![]
    }
}

fn count_pairs(root: Option<Rc<RefCell<TreeNode>>>, distance: i32) -> i32 {
    let mut count = 0;
    helper(root, distance, &mut count);

    count
}

pub fn main() {
    let root = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)];
    let distance = 3;
    println!("{}", count_pairs(vec_to_tree(root), 3));
}
