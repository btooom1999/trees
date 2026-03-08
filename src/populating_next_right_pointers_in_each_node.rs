use std::{cell::RefCell, rc::Rc};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
    next: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, left: None, right: None, next: None }
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

fn helper(
    node: Option<Rc<RefCell<TreeNode>>>,
    stack: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    depth: usize
) {
    if let Some(node_rc) = node {
        let mut node = node_rc.borrow_mut();
        if let Some(next) = stack[depth].clone() {
            node.next = Some(next.clone());
        }
        stack[depth] = Some(node_rc.clone());

        helper(node.right.clone(), stack, depth+1);
        helper(node.left.clone(), stack, depth+1);
    }
}

fn connect(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
   let mut stack = vec![None; 4096];
    helper(root.clone(), &mut stack, 0);

    root
}

pub fn main() {
    let root = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)];
    println!("{:?}", connect(vec_to_tree(root)));
}
