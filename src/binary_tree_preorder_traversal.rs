use std::{cell::RefCell, rc::Rc};

#[derive(Debug, Clone)]
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
                    new_nodes.push(Some(left.clone()));
                }

                i += 1;
            }

            // right side
            if i < nums.len() {
                if let Some(&val) = nums.get(i) && let Some(val) = val {
                    let right = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.right = Some(right.clone());
                    new_nodes.push(Some(right.clone()));
                }

                i += 1;
            }
        }

        nodes = new_nodes;
    }

    Some(root)
}

fn preorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();
    let mut stack = vec![];
    let mut curr = root;

    while let Some(node_rc) = curr {
        let node = node_rc.borrow();
        res.push(node.val);

        if node.right.is_some() {
            stack.push(node.right.clone());
        }

        if node.left.is_some() {
            curr = node.left.clone();
        } else if let Some(node) = stack.pop() {
            curr = node;
        } else {
            curr = None;
        }
    }

    res
}

pub fn main() {
    let root = [Some(1), None, Some(2), Some(3)];
    println!("{:#?}", preorder_traversal(vec_to_tree(root.into())));
}
