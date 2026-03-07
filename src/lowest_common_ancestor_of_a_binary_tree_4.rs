use std::{cell::RefCell, collections::{HashMap, HashSet}, rc::Rc};

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

type ReturnedTree = Option<Rc<RefCell<TreeNode>>>;
fn vec_to_tree(nums: Vec<Option<i32>>, p: i32, q: i32) -> (ReturnedTree, ReturnedTree, ReturnedTree) {
    if nums.is_empty() {
        return (None, None, None);
    }

    let root = Rc::new(RefCell::new(TreeNode::new(nums[0].unwrap())));
    let mut nodes = vec![root.clone()];
    let mut q_node = None;
    let mut p_node = None;
    if nums[0].unwrap() == p {
        p_node = Some(root.clone());
    }
    if nums[0].unwrap() == q {
        q_node = Some(root.clone());
    }

    let mut i = 1;
    while i < nums.len() {
        let mut new_nodes = Vec::new();
        for node_rc in nodes.iter() {
            let mut node = node_rc.borrow_mut();

            if let Some(&val) = nums.get(i) && let Some(val) = val {
                let left = Rc::new(RefCell::new(TreeNode::new(val)));
                node.left = Some(left.clone());
                new_nodes.push(left.clone());

                if val == p {
                    p_node = Some(left.clone());
                }
                if val == q {
                    q_node = Some(left.clone());
                }
            }
            i += 1;

            if let Some(&val) = nums.get(i) && let Some(val) = val {
                let right = Rc::new(RefCell::new(TreeNode::new(val)));
                node.right = Some(right.clone());
                new_nodes.push(right.clone());

                if val == p {
                    p_node = Some(right.clone());
                }
                if val == q {
                    q_node = Some(right.clone());
                }
            }
            i += 1;
        }

        nodes = new_nodes
    }

    (Some(root), p_node, q_node)
}

fn helper(
    node: Option<Rc<RefCell<TreeNode>>>,
    hashset: &HashSet<i32>,
    found: &mut Option<Rc<RefCell<TreeNode>>>,
) -> usize {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();
        let left = helper(node.left.clone(), hashset, found);
        let right = helper(node.right.clone(), hashset, found);
        let current = left + right + if hashset.contains(&node.val) { 1 } else { 0 };

        if current == hashset.len() {
            *found = Some(node_rc.clone());
            return 0;
        }

        current
    } else {
        0
    }
}

fn lowest_common_ancestor(root: Option<Rc<RefCell<TreeNode>>>, nodes: Vec<Option<Rc<RefCell<TreeNode>>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut res = None;
    let mut hashset = HashSet::new();
    for node_rc in nodes.iter().flatten() {
        hashset.insert(node_rc.borrow().val);
    }

    helper(root.clone(), &hashset, &mut res);

    res
}

pub fn main() {
    let root = vec![Some(3), Some(5), Some(1), Some(6), Some(2), Some(0), Some(8), None, None, Some(7), Some(4)];
    // let root = vec![Some(1), Some(2), Some(3), None, Some(4)];
    let p = 1;
    let q = 5;
    let v1 = Some(Rc::new(RefCell::new(TreeNode::new(7))));
    let v2 = Some(Rc::new(RefCell::new(TreeNode::new(41))));
    let (root, p, q) = vec_to_tree(root, p, q);

    println!("{:?}", lowest_common_ancestor(root, vec![p, q, v1, v2]));
}
