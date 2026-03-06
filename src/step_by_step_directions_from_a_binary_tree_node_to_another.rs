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

fn helper(
    node: Option<Rc<RefCell<TreeNode>>>,
    start_value: i32,
    dest_value: i32,
    start_str: &mut String,
    dest_str: &mut String,
) -> i32 {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();
        let left = helper(node.left.clone(), start_value, dest_value, start_str, dest_str);
        let right = helper(node.right.clone(), start_value, dest_value, start_str, dest_str);

        if (left == 1 && right == 2) || (left == 2 && right == 1) {
            start_str.push('U');
            if right == 2 { start_str.push('R'); } else { start_str.push('L'); }
            start_str.push_str(&*dest_str);
            return 0;
        }

        if left == 1 || right == 1 {
            start_str.push('U');
            if node.val == dest_value {
                start_str.push_str(&*dest_str);
                return 0;
            }

            return 1;
        }

        if left == 2 || right == 2 {
            *dest_str = if left == 2 { format!("L{}", *dest_str) } else { format!("R{}", *dest_str) };
            if node.val == start_value {
                start_str.push_str(&*dest_str);
                return 0;
            }

            return 2;
        }

        if node.val == start_value {
            return 1;
        }

        if node.val == dest_value {
            return 2;
        }

        0
    } else {
        0
    }
}

fn get_directions(root: Option<Rc<RefCell<TreeNode>>>, start_value: i32, dest_value: i32) -> String {
    let mut start_str = String::new();
    let mut dest_str = String::new();

    helper(root, start_value, dest_value, &mut start_str, &mut dest_str);

    start_str
}

pub fn main() {
    let root = vec![Some(5), Some(1), Some(2), Some(3), None, Some(6), Some(4)];
    // let root = vec![Some(2), Some(1)];
    let start_value = 3;
    let dest_value = 6;
    println!("{}", get_directions(vec_to_tree(root), start_value, dest_value));
}
