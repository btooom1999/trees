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

fn reverse_odd_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut nodes = vec![root.clone()];
    let mut row = 1;

    loop {
        if nodes.is_empty() {
            return root.clone();
        }

        let mut new_nodes = Vec::new();
        for (i, node_rc) in nodes.iter().flatten().enumerate() {
            let mut node = node_rc.borrow_mut();

            if row % 2 == 0 && i < nodes.len()-1-i {
                let reversed_node_rc = nodes[nodes.len()-1-i].as_ref().unwrap();
                let mut reversed_node = reversed_node_rc.borrow_mut();

                (node.val, reversed_node.val) = (reversed_node.val, node.val);
            }

            new_nodes.push(node.left.clone());
            new_nodes.push(node.right.clone());
        }

        nodes = new_nodes;
        row += 1;
    }
}

pub fn main() {
    let root = vec![Some(2), Some(3), Some(5), Some(8), Some(13), Some(21), Some(34)];
    // let root = vec![Some(2), Some(3), Some(5)];
    println!("{:?}", reverse_odd_levels(vec_to_tree(root)));
}
