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
        while i < nums.len() && let Some(val) = nums[i] {
            let child = Rc::new(TreeNode::new(val, None));
            node.push(child.clone());
            nodes.push_back(child.clone());
            i += 1;
        }
    }

    Some(root)
}

fn clone_tree(root: Option<Rc<TreeNode>>) -> Option<Rc<TreeNode>> {
    if let Some(node) = root {
        let new_node = Rc::new(TreeNode::new(node.val, None));
        let mut new_children = new_node.children.borrow_mut();

        for child in node.children.borrow().iter() {
            new_children.push(clone_tree(Some(child.clone())).unwrap());
        }

        Some(new_node.clone())
    } else {
        None
    }
}

pub fn main() {
    let root = vec![Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)];
    println!("{:#?}", clone_tree(vec_to_tree(root)));
}
