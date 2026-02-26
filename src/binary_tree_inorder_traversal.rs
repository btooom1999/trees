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
    let mut nodes = vec![Some(root.clone())];

    let mut i = 1;
    while i < nums.len() {
        let mut new_nodes = Vec::new();
        for node_rc in nodes.iter().flatten() {
            let mut node = node_rc.borrow_mut();

            // left side
            if i < nums.len() {
                if let Some(&val) = nums.get(i) && let Some(val) = val {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.left = Some(left.clone());
                    new_nodes.push(Some(left));
                } else {
                    new_nodes.push(None);
                }

                i += 1;
            }

            if i < nums.len() {
                if let Some(&val) = nums.get(i) && let Some(val) = val {
                    let right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    node.right = right.clone();
                    new_nodes.push(right);
                } else {
                    new_nodes.push(None);
                }

                i += 1;
            }
        }

        nodes = new_nodes;
    }

    Some(root)
}

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut stack = Vec::new();
    let mut curr = root;

    while let Some(node_rc) = curr {
        let mut node = node_rc.borrow_mut();

        if node.left.is_none() && node.right.is_none() {
            curr = stack.pop().flatten();
            res.push(node.val);
        } else if node.left.is_none() && node.right.is_some() {
            curr = node.right.take();
            res.push(node.val);
        } else if node.left.is_some() && node.right.is_none() {
            curr = node.left.take();
            stack.push(Some(node_rc.clone()));
        } else {
            curr = node.left.take();
            stack.push(node.right.take());
            stack.push(Some(node_rc.clone()));
        }
    }

    res
}

pub fn main() {
    let root = [Some(1), None, Some(2), Some(3)];
    println!("{:#?}", inorder_traversal(vec_to_tree(root.into())));
}
