use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    children: RefCell<Vec<Rc<TreeNode>>>
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, children: RefCell::new(vec![]) }
    }
}

fn vec_to_tree(nums: Vec<Option<i32>>) -> Option<Rc<TreeNode>> {
    if nums.is_empty() {
        return None;
    }

    let root = Rc::new(TreeNode::new(nums[0].unwrap()));
    let mut nodes = VecDeque::from([root.clone()]);

    let mut i = 1;
    let n = nums.len();
    while let Some(front) = nodes.pop_front() {
        i += 1;
        let mut children = front.children.borrow_mut();
        while i < n && let Some(val) = nums[i] {
            let node = Rc::new(TreeNode::new(val));
            nodes.push_back(node.clone());
            children.push(node);
            i += 1;
        }
    }

    Some(root)
}

fn helper(node: Rc<TreeNode>, depth: i32) -> i32 {
    let mut max = depth;
    for child in node.children.borrow().iter() {
        max = std::cmp::max(max, helper(child.clone(), depth+1));
    }

    max
}

fn max_depth(root: Option<Rc<TreeNode>>) -> i32 {
    root.map_or(0, |node| helper(node, 1))
}

pub fn main() {
    let root = [Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)];
    println!("{}", max_depth(vec_to_tree(root.into())));
}
