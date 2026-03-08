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

fn flip_equiv(root1: Option<Rc<RefCell<TreeNode>>>, root2: Option<Rc<RefCell<TreeNode>>>) -> bool {
    match (root1, root2) {
        (Some(node1_rc), Some(node2_rc)) => {
            let node1 = node1_rc.borrow();
            let node2 = node2_rc.borrow();

            if node1.val != node2.val {
                return false;
            }

            match (node1.left.clone(), node1.right.clone(), node2.left.clone(), node2.right.clone()) {
                (None, None, None, None) => true,
                (Some(left1_rc), Some(right1_rc), Some(left2_rc), Some(right2_rc)) => {
                    let left1 = left1_rc.borrow();
                    let right1 = right1_rc.borrow();
                    let left2 = left2_rc.borrow();
                    let right2 = right2_rc.borrow();

                    if left1.val == left2.val && right1.val == right2.val {
                        return flip_equiv(
                            Some(left1_rc.clone()),
                            Some(left2_rc.clone())
                        ) && flip_equiv(
                            Some(right1_rc.clone()),
                            Some(right2_rc.clone())
                        );
                    }

                    if left1.val == right2.val && right1.val == left2.val {
                        return flip_equiv(
                            Some(left1_rc.clone()),
                            Some(right2_rc.clone())
                        ) && flip_equiv(
                            Some(left2_rc.clone()),
                            Some(right1_rc.clone())
                        );
                    }

                    false
                },
                (Some(node1_rc), None, None, Some(node2_rc))
                | (None, Some(node1_rc), Some(node2_rc), _)
                | (Some(node1_rc), None, Some(node2_rc), _)
                | (None, Some(node1_rc), None, Some(node2_rc)) => {
                    let node1 = node1_rc.borrow();
                    let node2 = node2_rc.borrow();

                    if node1.val != node2.val {
                        return false;
                    }

                    flip_equiv(Some(node1_rc.clone()), Some(node2_rc.clone()))
                },
                _ => false

            }
        }
        (Some(_), _) | (_, Some(_)) => {
            false
        }
        _ => {
            true
        }
    }
}

pub fn main() {
    // let root1 = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), None, None, None, Some(7), Some(8)];
    // let root2 = vec![Some(1), Some(3), Some(2), None, Some(6), Some(4), Some(5), None, None, None, None, Some(8), Some(7)];
    let root1 = vec![Some(6), Some(1), Some(0)];
    let root2 = vec![Some(6), None, Some(1)];
    println!("{}", flip_equiv(vec_to_tree(root1), vec_to_tree(root2)));
}
