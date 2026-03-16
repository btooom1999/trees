use std::{cell::RefCell, collections::VecDeque, rc::Rc};

struct TreeNode {
    val: i32,
    children: RefCell<Vec<Rc<TreeNode>>>,
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
            children.push(node.clone());
            nodes.push_back(node);
            i += 1;
        }
    }

    Some(root)
}

fn helper(node: Rc<TreeNode>, vec: &mut Vec<i32>) {
    vec.push(node.val);
    for child in node.children.borrow().iter() {
        helper(child.clone(), vec);
    }
}

fn preorder(root: Option<Rc<TreeNode>>) -> Vec<i32> {
    root.map_or(Vec::new(), |node| {
        let mut res = Vec::new();
        helper(node, &mut res);

        res
    })
}

pub fn main() {
    let root = [Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)];
    println!("{:?}", preorder(vec_to_tree(root.into())));
}
