use std::{cell::RefCell, collections::BTreeMap, rc::Rc};

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

fn vec_to_nums(nums: Vec<Option<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
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

fn helper(node: Option<Rc<RefCell<TreeNode>>>, depth: i32, btreemap: &mut BTreeMap<i32, i32>) {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();
        btreemap.entry(depth).and_modify(|v| *v = std::cmp::max(*v, node.val)).or_insert(node.val);
        helper(node.left.clone(), depth+1, btreemap);
        helper(node.right.clone(), depth+1, btreemap);
    }
}

fn largest_values(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<i32> {
    let mut btreemap = BTreeMap::new();

    helper(root, 1, &mut btreemap);

    btreemap.into_values().collect()
}

pub fn main() {
    let root = vec![Some(1), Some(3), Some(2), Some(5), Some(3), None, Some(9)];
    println!("{:?}", largest_values(vec_to_nums(root)));
}
