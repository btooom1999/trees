use std::{cell::RefCell, rc::Rc};

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

fn dfs(nums: &[i32]) -> Option<Rc<RefCell<TreeNode>>> {
    if nums.is_empty() { return None };
    let idx = nums.iter().enumerate().max_by_key(|x| x.1).unwrap().0;
    let mut root = TreeNode::new(nums[idx]);
    root.left = dfs(&nums[..idx]);
    root.right = dfs(&nums[idx+1..]);

    Some(Rc::new(RefCell::new(root)))
}

fn construct_maximum_binary_tree(nums: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    dfs(&nums)
}

pub fn main() {
    let nums = [3,2,1,6,0,5].to_vec();
    println!("{:?}", construct_maximum_binary_tree(nums));
}
