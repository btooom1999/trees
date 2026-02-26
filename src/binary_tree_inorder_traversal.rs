use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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
    let mut nodes = vec![Some(root.clone())];

    let mut i = 1;
    while i < nums.len() {
        let mut new_nodes = Vec::new();
        for node_rc in nodes.iter().flatten() {
            let mut node = node_rc.borrow_mut();

            // left side
            if i < nums.len() {
                if let Some(&val) = nums.get(i) && let Some(val) = val {
                    let left = Rc::new(RefCell::new(TreeNode::new(val)));
                    node.left = Some(left.clone());
                    new_nodes.push(Some(left));
                } else {
                    new_nodes.push(None);
                }

                i += 1;
            }

            if i < nums.len() {
                if let Some(&val) = nums.get(i) && let Some(val) = val {
                    let right = Some(Rc::new(RefCell::new(TreeNode::new(val))));
                    node.right = right.clone();
                    new_nodes.push(right);
                } else {
                    new_nodes.push(None);
                }

                i += 1;
            }
        }

        nodes = new_nodes;
    }

    Some(root)
}

fn inorder_traversal(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut res = Vec::new();

    fn helper(node: Option<Rc<RefCell<TreeNode>>>, res: &mut Vec<i32>) {
        if let Some(node_rc) = node {
            let node = node_rc.borrow();
            helper(node.left.clone(), res);
            res.push(node.val);
            helper(node.right.clone(), res);
        }
    }

    helper(root, &mut res);

    res
}

pub fn main() {
    let root = [Some(1), None, Some(2), Some(3)];
    println!("{:#?}", inorder_traversal(vec_to_tree(root.into())));
}
