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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, max: i32, count: &mut i32) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();
        let max = std::cmp::max(max, node.val);
        if node.val >= max {
            *count += 1;
        }

        helper(node.left.clone(), max, count);
        helper(node.right.clone(), max, count);
    }
}

fn good_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut count = 0;
    helper(root, -10_001, &mut count);

    count
}

pub fn main() {
    let root = vec![Some(3), Some(1), Some(4), Some(3), None, Some(1), Some(5)];
    println!("{}", good_nodes(vec_to_tree(root)));
}
