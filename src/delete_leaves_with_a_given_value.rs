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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, target: i32) -> bool {
    if let Some(node_rc) = node {
        let mut node = node_rc.borrow_mut();
        let left = helper(node.left.clone(), target);
        let right = helper(node.right.clone(), target);
        if left {
            node.left = None;
        }
        if right {
            node.right = None;
        }

        node.left.is_none() && node.right.is_none() && node.val == target
    } else {
        false
    }
}

fn remove_leaf_nodes(root: Option<Rc<RefCell<TreeNode>>>, target: i32) -> Option<Rc<RefCell<TreeNode>>> {
    let node_rc = Rc::new(RefCell::new(TreeNode { val: 0, left: root, right: None }));
    helper(Some(node_rc.clone()), target);

    node_rc.borrow().left.clone()
}

pub fn main() {
    let root = vec![Some(1), Some(2), Some(3), Some(2), None, Some(2), Some(4)];
    println!("{:?}", remove_leaf_nodes(vec_to_tree(root), 2));
}
