use std::{cell::RefCell, collections::HashSet, rc::Rc};

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
    hashset: &mut HashSet<i32>,
    target: i32,
) -> bool {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();
        if hashset.contains(&(target - node.val)) {
            return true;
        }

        hashset.insert(node.val);

        helper(node.left.clone(), hashset, target) || helper(node.right.clone(), hashset, target)
    } else {
        false
    }
}

fn find_target(root: Option<Rc<RefCell<TreeNode>>>, k: i32) -> bool {
    let mut hashset = HashSet::new();

    helper(root, &mut hashset, k)
}

pub fn main() {
    let root = [Some(5), Some(3), Some(6), Some(2), Some(4), None, Some(7)];
    let k = 9;
    println!("{}", find_target(vec_to_tree(root.into()), k));
}
