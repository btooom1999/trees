use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    children: RefCell<Vec<Rc<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32, children: Option<RefCell<Vec<Rc<TreeNode>>>>) -> Self {
        Self { val, children: children.unwrap_or_default() }
    }
}

fn vec_to_tree(nums: Vec<Option<i32>>) -> Option<Rc<TreeNode>> {
    if nums.is_empty() {
        return None;
    }

    let root = Rc::new(TreeNode::new(nums[0].unwrap(), None));
    let mut nodes = VecDeque::from([root.clone()]);

    let mut i = 1;
    while i < nums.len() && let Some(front) = nodes.pop_front() {
        i += 1;
        let mut node = front.children.borrow_mut();
        while i < nums.len() && let Some(num) = nums[i] {
            let mut child = Rc::new(TreeNode::new(num, None));
            node.push(child.clone());
            nodes.push_back(child.clone());
            i += 1;
        }
    }

    Some(root)
}

fn helper(node: Option<Rc<TreeNode>>, vec: &mut Vec<i32>) {
    if let Some(node) = node {
        let children = node.children.borrow();
        for child in children.iter() {
            helper(Some(child.clone()), vec);
        }

        vec.push(node.val);
    }
}

fn recursive_postorder(root: Option<Rc<TreeNode>>) -> Vec<i32> {
    let mut vec = Vec::new();
    helper(root, &mut vec);

    vec
}

fn iterative_postorder(root: Option<Rc<TreeNode>>) -> Vec<i32> {
    if root.is_none() {
        return vec![];
    }

    let mut nodes = vec![root.unwrap()];
    let mut res = VecDeque::new();
    while let Some(node) = nodes.pop() {
        res.push_front(node.val);
        nodes.extend(node.children.borrow().clone());
    }

    res.into()
}

pub fn main() {
    // let root = vec![Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)];
    let root = vec![
        Some(1), None, Some(2), Some(3), Some(4), Some(5), None, None,
        Some(6), Some(7), None, Some(8), None, Some(9), Some(10), None, None,
        Some(11), None, Some(12), None, Some(13), None, None, Some(14)
    ];
    println!("{:?}", iterative_postorder(vec_to_tree(root)));
}
