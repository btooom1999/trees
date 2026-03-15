use std::{cell::RefCell, rc::Rc};

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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, sum: &mut i32) {
    if let Some(node_rc) = node {
        let mut node = node_rc.borrow_mut();

        helper(node.right.clone(), sum);
        node.val += *sum;
        *sum = node.val;
        helper(node.left.clone(), sum);
    }
}

fn convert_bst(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    helper(root.clone(), &mut 0);

    root
}

pub fn main() {
    let root = vec![Some(4), Some(1), Some(6), Some(0), Some(2), Some(5), Some(7), None, None, None, Some(3), None, None, None, Some(8)];
    println!("{:?}", convert_bst(vec_to_tree(root)));
}
