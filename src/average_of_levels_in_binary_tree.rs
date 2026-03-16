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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, depth: usize, vec: &mut Vec<(i32, i32)>) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        if let Some(value) = vec.get_mut(depth) {
            value.0 += node.val;
            value.1 += 1;
        } else {
            vec.push((node.val, 1));
        }

        helper(node.left.clone(), depth+1, vec);
        helper(node.right.clone(), depth+1, vec);
    }
}

fn average_of_levels(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<f64> {
    let mut res = Vec::new();
    helper(root, 0, &mut res);

    res.into_iter().map(|(sum, n)| sum as f64 / n as f64).collect()
}

pub fn main() {
    let root = [Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    println!("{:?}", average_of_levels(vec_to_tree(root.into())));
}
