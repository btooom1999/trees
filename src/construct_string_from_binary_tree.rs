use std::{cell::RefCell, fmt::format, rc::{self, Rc}};

struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, left: None, right: None, }
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
                new_nodes.push(left);
            }
            i += 1;

            if let Some(&val) = nums.get(i) && let Some(val) = val {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.right = Some(right.clone());
                new_nodes.push(right);
            }
            i += 1;
        }

        nodes = new_nodes
    }

    Some(root)
}

fn helper(node: Option<Rc<RefCell<TreeNode>>>) -> Option<String> {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();
        match (helper(node.right.clone()), helper(node.left.clone())) {
            (Some(val2), Some(val1)) => {
                Some(format!("({}{}{})", node.val, val1, val2))
            }
            (Some(val), _) => {
                Some(format!("({}(){})", node.val, val))
            },
            (_, Some(val)) => {
                Some(format!("({}{})", node.val, val))
            },
            _ => {
                Some(format!("({})", node.val))
            },
        }
    } else {
        None
    }
}

fn tree2str(root: Option<Rc<RefCell<TreeNode>>>) -> String {
    if root.is_none() {
        return String::new();
    }

    let root = root.unwrap();
    let node = root.borrow();
    match (node.left.clone(), node.right.clone()) {
       (Some(left), Some(right)) => {
            let str1 = helper(Some(left.clone()));
            let str2 = helper(Some(right.clone()));
            format!("{}{}{}", node.val, str1.unwrap(), str2.unwrap())
        }
        (Some(n), _) => {
            let str = helper(Some(n.clone()));
            format!("{}{}", node.val, str.unwrap())
        }
        (_, Some(n)) => {
            let str = helper(Some(n.clone()));
            format!("{}(){}", node.val, str.unwrap())
        }
        _ => { format!("{}", node.val) }
    }
}

pub fn main() {
    let root = vec![Some(1), None, Some(2), None, Some(3), None, Some(4), None, Some(5), None, Some(6)];
    // let root = vec![Some(1), Some(2), Some(3), None, Some(5), Some(6), Some(7), Some(8), Some(9), None, Some(10), Some(11)];
    println!("{}", tree2str(vec_to_tree(root)));
}
