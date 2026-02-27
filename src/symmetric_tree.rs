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

        nodes = new_nodes;
    }

    Some(root)
}

fn is_symmetric(root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    root.is_some_and(|node_rc| {
        let mut stack = Vec::new();
        let mut node = node_rc.borrow_mut();

        let mut left = node.left.take();
        let mut right = node.right.take();

        if (left.is_none() && right.is_some()) || (left.is_some() && right.is_none()) {
            return false;
        }

        while let (Some(node1_rc), Some(node2_rc)) = (left, right) {
            let mut node1 = node1_rc.borrow_mut();
            let mut node2 = node2_rc.borrow_mut();

            if node1.val != node2.val {
                return false;
            }

            match (node1.left.take(), node1.right.take(), node2.left.take(), node2.right.take()) {
                (Some(l1), Some(r1), Some(l2), Some(r2)) => {
                    left = Some(l1);
                    right = Some(r2);
                    stack.push((Some(l2), Some(r1)));
                }
                (None, None, None, None) => {
                    if let Some((node1, node2)) = stack.pop() {
                        left = node1;
                        right = node2;
                    } else {
                        return true;
                    }
                }
                (Some(n1), None, None, Some(n2))
                | (None, Some(n1), Some(n2), None) => {
                    left = Some(n1);
                    right = Some(n2);
                }
                _ => { return false; }
            }
        }

        true
    })
}

pub fn main() {
    let root = vec![Some(1), Some(2), Some(2), Some(3), Some(4), Some(4), Some(3)];
    println!("{}", is_symmetric(vec_to_tree(root)));
}
