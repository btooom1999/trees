use std::{cell::RefCell, collections::{HashSet, VecDeque}, rc::{Rc, Weak}};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
    parent: Option<Weak<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, left: None, right: None, parent: None }
    }
}

type TripleTreeNode = Option<Rc<RefCell<TreeNode>>>;
fn vec_to_tree(nums: Vec<Option<i32>>, p: i32, q: i32) -> (TripleTreeNode, TripleTreeNode, TripleTreeNode) {
    if nums.is_empty() {
        return (None, None, None);
    }

    let root = Rc::new(RefCell::new(TreeNode::new(nums[0].unwrap())));
    let mut nodes = VecDeque::from([root.clone()]);
    let mut q_node = None;
    let mut p_node = None;
    if p == nums[0].unwrap() {
        p_node = Some(root.clone());
    } else if q == nums[0].unwrap() {
        q_node = Some(root.clone());
    }

    let mut i = 1;
    while let Some(front) = nodes.pop_front() {
        let mut node = front.borrow_mut();

        if let Some(&val) = nums.get(i) && let Some(val) = val {
            let mut new_node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.left = Some(new_node.clone());
            nodes.push_back(new_node.clone());
            new_node.borrow_mut().parent = Some(Rc::downgrade(&front.clone()));
            if p == val {
                p_node = Some(new_node.clone());
            } else if q == val {
                q_node = Some(new_node.clone());
            }
        }
        i += 1;

        if let Some(&val) = nums.get(i) && let Some(val) = val {
            let new_node = Rc::new(RefCell::new(TreeNode::new(val)));
            node.right = Some(new_node.clone());
            nodes.push_back(new_node.clone());
            new_node.borrow_mut().parent = Some(Rc::downgrade(&front.clone()));
            if p == val {
                p_node = Some(new_node.clone());
            } else if q == val {
                q_node = Some(new_node.clone());
            }
        }
        i += 1;
    }

    (Some(root), p_node, q_node)
}

fn helper(
    node: Option<Rc<RefCell<TreeNode>>>,
    condition: i32,
    hashset: &mut HashSet<i32>,
) -> Option<Rc<RefCell<TreeNode>>> {
    if let Some(node_rc) = node {
        let mut node = node_rc.borrow();

        if hashset.contains(&node.val) ||  node.val == condition {
            return Some(node_rc.clone());
        } else {
            hashset.insert(node.val);
        }

        if let Some(parent) = node.parent.clone() {
            let parent = Weak::upgrade(&parent);
            helper(parent, condition, hashset)
        } else {
            None
        }
    } else {
        None
    }
}

fn lowest_common_ancestor(p: Option<Rc<RefCell<TreeNode>>>, q: Option<Rc<RefCell<TreeNode>>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut hashset = HashSet::new();

    if let Some(val) = helper(p.clone(), q.as_ref().unwrap().borrow().val, &mut hashset) {
        Some(val)
    } else {
        helper(q, p.unwrap().borrow().val, &mut hashset)
    }
}

pub fn main() {
    // let root = vec![Some(5), Some(3), Some(4), Some(2), Some(1)];
    let root = vec![
        Some(5), Some(3), Some(4), Some(2), Some(1),
        None, Some(9),
        None, Some(11), Some(10), Some(12)
    ];
    let p = 3;
    let q = 12;
    let (root, p, q) = vec_to_tree(root, p, q);
    println!("{:?}", lowest_common_ancestor(p, q));
}
