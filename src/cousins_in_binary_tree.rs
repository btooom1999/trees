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

        nodes = new_nodes
    }

    Some(root)
}

fn helper(
    node: Option<Rc<RefCell<TreeNode>>>,
    res1: &mut Option<(i32, i32)>,
    res2: &mut Option<(i32, i32)>,
    x: i32,
    y: i32,
    parent: i32,
    depth: i32,
) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();
        if node.val == x {
            *res1 = Some((depth, parent));
        }
        if node.val == y {
            *res2 = Some((depth, parent));
        }

        helper(node.left.clone(), res1, res2, x, y, node.val, depth+1);
        helper(node.right.clone(), res1, res2, x, y, node.val, depth+1);
    }
}

fn is_cousins(root: Option<Rc<RefCell<TreeNode>>>, x: i32, y: i32) -> bool {
    let (mut res1, mut res2) = (None, None);
    helper(root, &mut res1, &mut res2, x, y, -1, 0);

    println!("{:?} {:?}", res1, res2);

    if let (Some(res1), Some(res2)) = (res1, res2) {
        res1.0 == res2.0 && res1.1 != res2.1
    } else {
        false
    }
}

pub fn main() {
    let root = vec![Some(1), Some(2), Some(3), Some(4)];
    let (x, y) = (4, 3);
    println!("{}", is_cousins(vec_to_tree(root), x, y))
}
