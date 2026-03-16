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
    let n = nums.len();
    while i < n {
        let mut new_nodes = Vec::new();
        for node_rc in nodes.iter() {
            let mut node = node_rc.borrow_mut();

            if i < n && let Some(val) = nums[i] {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(left.clone());
                new_nodes.push(left.clone());
            }
            i += 1;

            if i < n && let Some(val) = nums[i] {
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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, max: &mut i32) -> VecDeque<(i32, i32)> {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        let mut left = helper(node.left.clone(), max);
        let mut right = helper(node.right.clone(), max);

        let item = if let (Some(back), Some(front)) = (left.back(), right.front()) && back.0 == node.val && front.0 == node.val {
            (node.val, left.pop_back().unwrap().1 + right.pop_front().unwrap().1 + 1)
        } else if let Some(back) = left.back() && back.0 == node.val {
            (node.val, left.pop_back().unwrap().1 + 1)
        } else if let Some(front) = right.front() && front.0 == node.val {
            (node.val, right.pop_front().unwrap().1 + 1)
        } else {
            (node.val, 1)
        };

        *max = std::cmp::max(*max, item.1);
        let mut stack = VecDeque::new();
        stack.extend(left);
        stack.push_back(item);
        stack.extend(right);
        stack
    } else {
        VecDeque::new()
    }
}

fn find_mode(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut max = 0;
    let mut stack = helper(root, &mut max);

    stack.iter().filter_map(|v| if v.1 == max { Some(v.0) } else { None }).collect()
}

pub fn main() {
    let root = [Some(1), None, Some(2), Some(2)];
    // let root = [Some(1), Some(1), Some(1), None, None, Some(1), Some(2)];
    println!("{:?}", find_mode(vec_to_tree(root.into())));
}
