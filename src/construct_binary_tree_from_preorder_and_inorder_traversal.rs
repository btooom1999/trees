use std::{cell::RefCell, collections::VecDeque, rc::{self, Rc}, usize};

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

fn helper(preorder: &Vec<i32>, inorder: &Vec<i32>, i: usize, start: usize, end: usize) -> (Option<Rc<RefCell<TreeNode>>>, usize) {
    if start == end {
        return (None, i);
    }

    let val = preorder[i];
    let idx = inorder.iter().position(|v| *v == val).unwrap();

    let root = Rc::new(RefCell::new(TreeNode::new(val)));
    let (left, i) = helper(preorder, inorder, i+1, start, idx);
    root.borrow_mut().left = left;
    let (right, i) = helper(preorder, inorder, i, idx+1, end);
    root.borrow_mut().right = right;

    (Some(root), i)
}

fn build_tree(preorder: Vec<i32>, inorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    helper(&preorder, &inorder, 0, 0, inorder.len()).0
}

pub fn main() {
    let preorder = [3,9,20,15,7];
    let inorder = [9,3,15,20,7];
    // let preorder = [1,2];
    // let inorder = [1,2];
    println!("{:?}", build_tree(preorder.into(), inorder.into()));
}
