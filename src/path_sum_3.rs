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

fn helper(
    node: Option<Rc<RefCell<TreeNode>>>,
    mut nums: VecDeque<i64>,
    target_sum: i64,
    sum: i64,
    count: &mut i32,
) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        nums.push_back(node.val as i64);
        let sum = sum + node.val as i64;

        let mut temp_sum = sum;
        let mut temp_nums = nums.clone();

        while !temp_nums.is_empty() {
            if temp_sum == target_sum {
                *count += 1;
            }

            temp_sum -= temp_nums.pop_front().unwrap();
        }

        helper(node.left.clone(), nums.clone(), target_sum, sum, count);
        helper(node.right.clone(), nums, target_sum, sum, count);
    }
}

fn path_sum(root: Option<Rc<RefCell<TreeNode>>>, target_sum: i32) -> i32 {
    let mut count = 0;
    helper(root, VecDeque::new(), target_sum as i64, 0, &mut count);

    count
}

pub fn main() {
    let root = vec![Some(10), Some(5), Some(-3), Some(3), Some(2), None, Some(11), Some(3), Some(-2), None, Some(1)];
    let target_sum = 8;
    println!("{:?}", path_sum(vec_to_tree(root), target_sum))
}
