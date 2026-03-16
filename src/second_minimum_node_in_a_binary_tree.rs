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

    while i < n {
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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, first: &mut Option<i32>, second: &mut Option<i32>) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        if let Some(first) = first {
            if node.val <= *first {
                if node.val != *first { *second = Some(*first); }
                *first = node.val;
            } else if let Some(second) = second {
                if node.val < *second {
                    *second = node.val
                }
            } else {
                *second = Some(node.val);
            }
        } else {
            *first = Some(node.val);
        }

        helper(node.left.clone(), first, second);
        helper(node.right.clone(), first, second);
    }
}

fn find_second_minimum_value(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut first = None;
    let mut second = None;
    helper(root, &mut first, &mut second);

    second.unwrap_or(-1)
}

pub fn main() {
    let root: Vec<Option<i32>> = vec![Some(2), Some(2), Some(5), None, None, Some(5), Some(7)];
    println!("{}", find_second_minimum_value(vec_to_tree(root)));
}
