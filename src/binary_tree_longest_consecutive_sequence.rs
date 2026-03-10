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

fn vec_to_tree(vec: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    if vec.is_empty() {
        return None;
    }

    let root = Rc::new(RefCell::new(TreeNode::new(vec[0].unwrap())));
    let mut nodes = vec![root.clone()];

    let mut i = 1;
    while i < vec.len() {
        let mut new_nodes = Vec::new();
        for node_rc in nodes.iter() {
            let mut node = node_rc.borrow_mut();

            if let Some(&val) = vec.get(i) && let Some(val) = val {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(left.clone());
                new_nodes.push(left.clone());
            }
            i += 1;

            if let Some(&val) = vec.get(i) && let Some(val) = val {
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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, long: i32, max_long: &mut i32) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        if let Some(left) = node.left.clone() {
            let left_node = left.borrow();
            helper(node.left.clone(), if node.val + 1 == left_node.val { long + 1 } else { 1 }, max_long);
        }

        if let Some(right) = node.right.clone() {
            let right_node = right.borrow();
            helper(node.right.clone(), if node.val + 1 == right_node.val { long + 1 } else { 1 }, max_long);
        }

        if long > 1 {
            *max_long = std::cmp::max(*max_long, long);
        }
    }
}

fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut max_long = -1;
    helper(root, 1, &mut max_long);

    max_long
}

pub fn main() {
    let root = vec![Some(1), None, Some(3), Some(2), Some(4), None, None, None, Some(5)];
    // let root1: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3)];
    // let root2: Vec<Option<i32>> = vec![Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)];
    // let root3: Vec<Option<i32>> = vec![Some(1), None, Some(2), None, Some(3), None, Some(4)];
    // let root4: Vec<Option<i32>> = vec![Some(3), Some(2), Some(4), None, None, None, Some(5)];
    // let root5: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3), None, None, Some(4), Some(5)];
    // let tree_vec: Vec<Option<i32>> = vec![Some(10), Some(20), Some(30), Some(40), None, Some(60), None];
    println!("{}", longest_consecutive(vec_to_tree(root)));
}
