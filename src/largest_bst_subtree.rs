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

fn helper(node: Option<Rc<RefCell<TreeNode>>>) -> (i32, i32, i32) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        if node.left.is_none() && node.right.is_none() {
            return (node.val, node.val, 1);
        }

        let left = helper(node.left.clone());
        let right = helper(node.right.clone());

        let mut ans = (0, 0, 0);
        if node.val > left.1 && node.val < right.0 {
            ans.0 = std::cmp::min(left.0, node.val);
            ans.1 = std::cmp::max(right.1, node.val);
            ans.2 = left.2 + right.2 + 1;
            return ans;
        }

        ans.0 = i32::MIN;
        ans.1 = i32::MAX;
        ans.2 = std::cmp::max(left.2, right.2);

        ans
    } else {
        (i32::MAX, i32::MIN, 0)
    }
}

fn largest_bst_subtree(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    root.map_or(0, |node_rc| {
        let node = node_rc.borrow();
        let left = helper(node.left.clone());
        let right = helper(node.right.clone());

        std::cmp::max(left.2, right.2)
    })
}

pub fn main() {
    let root = vec![Some(5), Some(2), Some(4), Some(1), Some(3)];
    println!("{}", largest_bst_subtree(vec_to_tree(root)));
}
