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

fn helper(
    node: Option<Rc<RefCell<TreeNode>>>,
    longest: &mut i32,
) -> (i32, i32) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();

        let (l_inc, l_dec) = helper(node.left.clone(), longest);
        let (r_inc, r_dec) = helper(node.right.clone(), longest);

        let (mut inc, mut dec) = (1, 1);

        if let Some(left) = node.left.clone() {
            let val = left.borrow().val;
            if node.val - 1 == val {
                dec = std::cmp::max(dec, l_dec + 1);
            } else if node.val + 1 == val {
                inc = std::cmp::max(inc, l_inc + 1);
            }
        }

        if let Some(right) = node.right.clone() {
            let val = right.borrow().val;
            if node.val - 1 == val {
                dec = std::cmp::max(dec, r_dec + 1);
            } else if node.val + 1 == val {
                inc = std::cmp::max(inc, r_inc + 1);
            }
        }

        *longest = std::cmp::max(*longest, inc + dec - 1);

        (inc, dec)
    } else {
        (0, 0)
    }
}

fn longest_consecutive(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut longest = 0;
    helper(root, &mut longest);
    longest
}

pub fn main() {
    // let root = vec![Some(2), Some(1), Some(3)];
    let root = vec![Some(3), None, Some(2), Some(1)];
    // let root = vec![Some(4), Some(5), Some(3)];
    println!("{:?}", longest_consecutive(vec_to_tree(root)));
}
