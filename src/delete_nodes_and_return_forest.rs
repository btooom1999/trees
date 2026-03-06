use std::{cell::RefCell, collections::HashSet, rc::Rc};

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
    hashset: &HashSet<i32>,
    vec: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
) -> bool {
    if let Some(node_rc) = node {
        let mut node = node_rc.borrow_mut();
        let left = helper(node.left.clone(), hashset, vec);
        let right = helper(node.right.clone(), hashset, vec);
        let exist = hashset.contains(&node.val);

        if left {
            node.left = None;
        }
        if right {
            node.right = None;
        }

        if exist {
            if node.left.is_some() { vec.push(node.left.take()); }
            if node.right.is_some() { vec.push(node.right.take()); }
        }

        exist
    } else {
        false
    }
}

fn del_nodes(root: Option<Rc<RefCell<TreeNode>>>, to_delete: Vec<i32>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let new_root = Rc::new(RefCell::new(TreeNode { val: 0, left: root, right: None }));
    let hashset = to_delete.into_iter().collect::<HashSet<_>>();
    let mut res = Vec::new();
    helper(Some(new_root.clone()), &hashset, &mut res);

    if new_root.borrow().left.is_some() {
        res.push(new_root.borrow().left.clone());
    }

    res
}

pub fn main() {
    let root = vec![Some(1), Some(2), Some(3), Some(4), Some(5), Some(6), Some(7)];
    let to_delete = [3, 5];
    println!("{:?}", del_nodes(vec_to_tree(root), to_delete.into()));
}
