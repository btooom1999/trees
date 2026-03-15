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

fn flatten(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut last = None as Option<Rc<RefCell<TreeNode>>>;
    let mut cur = root.clone();

    while let Some(node_rc) = cur {
        let mut node = node_rc.borrow_mut();

        if node.left.is_some() {
            let right = node.right.take();
            let mut last_right = right.clone();
            while let Some(right_rc) = last_right {
                if right_rc.borrow_mut().right.is_some() {
                    last_right = right_rc.borrow().right.clone();
                } else {
                    right_rc.borrow_mut().right = last;
                    last = right;
                    break;
                }
            }

            node.right = node.left.take();
        }

        if node.right.is_none() {
            node.right = last;
            last = None;
        }

        cur = node.right.clone();
    }
}

pub fn main() {
    let mut root = vec_to_tree([Some(1), Some(2), Some(5), Some(3), Some(4), None, Some(6)].into());
    flatten(&mut root);

    println!("{:#?}", root);
}
