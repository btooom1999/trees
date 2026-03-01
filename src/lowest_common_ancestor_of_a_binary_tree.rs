use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};

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

type ReturnedTree = Option<Rc<RefCell<TreeNode>>>;
fn vec_to_tree(nums: Vec<Option<i32>>, p: i32, q: i32) -> (ReturnedTree, ReturnedTree, ReturnedTree) {
    if nums.is_empty() {
        return (None, None, None);
    }

    let root = Rc::new(RefCell::new(TreeNode::new(nums[0].unwrap())));
    let mut nodes = vec![root.clone()];
    let mut q_node = None;
    let mut p_node = None;
    if nums[0].unwrap() == p {
        p_node = Some(root.clone());
    }
    if nums[0].unwrap() == q {
        q_node = Some(root.clone());
    }

    let mut i = 1;
    while i < nums.len() {
        let mut new_nodes = Vec::new();
        for node_rc in nodes.iter() {
            let mut node = node_rc.borrow_mut();

            if let Some(&val) = nums.get(i) && let Some(val) = val {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(left.clone());
                new_nodes.push(left.clone());

                if val == p {
                    p_node = Some(left.clone());
                }
                if val == q {
                    q_node = Some(left.clone());
                }
            }
            i += 1;

            if let Some(&val) = nums.get(i) && let Some(val) = val {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.right = Some(right.clone());
                new_nodes.push(right.clone());

                if val == p {
                    p_node = Some(right.clone());
                }
                if val == q {
                    q_node = Some(right.clone());
                }
            }
            i += 1;
        }

        nodes = new_nodes
    }

    (Some(root), p_node, q_node)
}

fn helper(
    node: Option<Rc<RefCell<TreeNode>>>,
    p: i32,
    q: i32,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        if node.val == p || node.val == q {
            return Some(node_rc.clone());
        }

        let left = helper(node.left.clone(), p, q);
        let right = helper(node.right.clone(), p, q);

        if left.is_some() && right.is_some() {
            return Some(node_rc.clone());
        }

        if left.is_some() { left } else { right }
    } else {
        None
    }
}

fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let (p, q) = (p.unwrap().borrow().val, q.unwrap().borrow().val);

    helper(root, p, q)
}

pub fn main() {
    let root = vec![Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8), None, None, Some(7), Some(4)];
    // let root = vec![Some(1), Some(2), Some(3), None, Some(4)];
    let p = 5;
    let q = 1;
    let (root, p, q) = vec_to_tree(root, p, q);

    println!("{:?}", lowest_common_ancestor(root, p, q));
}
