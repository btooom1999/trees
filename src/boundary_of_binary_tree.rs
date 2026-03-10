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

        nodes = new_nodes;
    }

    Some(root)
}

fn helper(
    node: Option<Rc<RefCell<TreeNode>>>,
    boundary: &mut VecDeque<i32>,
    leaf_nodes: &mut VecDeque<i32>,
    direction: u8,
) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();
        if node.left.is_none() && node.right.is_none() {
            leaf_nodes.push_back(node.val);
        } else if direction == b'l' {
            boundary.push_back(node.val);
        } else if direction == b'r' {
            boundary.push_front(node.val);
        }

        if direction == b'l' {
            helper(node.left.clone(), boundary, leaf_nodes, direction);
            helper(node.right.clone(), boundary, leaf_nodes,  if node.left.is_some() { b'm' } else { b'l' });
        } else if direction == b'r' {
            helper(node.left.clone(), boundary, leaf_nodes, if node.right.is_some() { b'm' } else { b'r' });
            helper(node.right.clone(), boundary, leaf_nodes, direction);
        } else {
            helper(node.left.clone(), boundary, leaf_nodes, direction);
            helper(node.right.clone(), boundary, leaf_nodes, direction);
        }
    }
}

fn boundary_of_binary_tree(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    match root {
        Some(node_rc) => {
            let node = node_rc.borrow();
            let mut left_boundary = VecDeque::new();
            let mut right_boundary = VecDeque::new();
            let mut leaf_nodes = VecDeque::new();

            helper(node.left.clone(), &mut left_boundary, &mut leaf_nodes, b'l');
            helper(node.right.clone(), &mut right_boundary, &mut leaf_nodes, b'r');

            let mut res = vec![node.val];
            res.extend(left_boundary);
            res.extend(leaf_nodes);
            res.extend(right_boundary);

            res
        },
        _ => vec![]
    }
}

pub fn main() {
    let root = vec![
        Some(1), Some(2), Some(3),
        Some(4), Some(5), Some(6), Some(7),
        None, None,
        Some(8), Some(9),
    ];
    // let root = vec![Some(1), None, Some(2), None, Some(3), None, Some(4)];
    println!("{:?}", boundary_of_binary_tree(vec_to_tree(root)));
}
