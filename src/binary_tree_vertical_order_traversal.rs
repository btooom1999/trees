use std::{cell::RefCell, collections::{HashMap, VecDeque}, rc::Rc};

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

fn vertical_order(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Vec<i32>> {
    if root.is_none() {
        return vec![];
    }

    let mut hashmap = HashMap::<i32, Vec<i32>>::new();
    let mut res = Vec::new();

    let mut min_index = 0;
    let mut nodes = VecDeque::new();
    nodes.push_back((0, root.clone().unwrap()));

    loop {
        if nodes.is_empty() {
            break;
        }

        let mut new_nodes = VecDeque::new();
        while let Some((index, node_rc)) = nodes.pop_front() {
            let node = node_rc.borrow();
            hashmap.entry(index).or_default().push(node.val);
            min_index = min_index.min(index);
            if let Some(left) = node.left.clone() {
                new_nodes.push_back((index-1, left.clone()));
            }
            if let Some(right) = node.right.clone() {
                new_nodes.push_back((index+1, right.clone()));
            }
        }
        nodes = new_nodes;
    }

    while let Some(vec) = hashmap.remove(&min_index) {
        res.push(vec);
        min_index += 1;
    }

    res
}

pub fn main() {
    // let root = vec![Some(3), Some(9), Some(20), None, None, Some(15), Some(7)];
    let root: Vec<Option<i32>> = vec![Some(3), Some(9), Some(8), Some(4), Some(0), Some(1), Some(7)];
    // let root: Vec<Option<i32>> = vec![Some(1), Some(2), Some(3), Some(4), Some(10), Some(9), Some(11), None, Some(5), None, None, None, None, None, None, None, Some(6)];
    println!("{:?}", vertical_order(vec_to_tree(root)));
}
