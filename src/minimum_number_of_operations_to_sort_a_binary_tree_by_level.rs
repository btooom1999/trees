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

fn helper(count: &mut i32, mut level: Vec<i32>) {
    let mut sorted_level = level.clone();
    sorted_level.sort();

    let mut hashmap = level.clone().into_iter().zip(0..).collect::<HashMap<i32, usize>>();
    for i in 0..level.len() {
        if level[i] != sorted_level[i] {
            let j = hashmap.get(&sorted_level[i]).unwrap().to_owned();
            hashmap.entry(level[i]).and_modify(|v| *v = j);
            hashmap.entry(sorted_level[i]).and_modify(|v| *v = i);

            (level[i], level[j]) = (level[j], level[i]);
            *count += 1;
        }
    }
}

fn minimum_operations(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    let mut level = vec![];
    let mut nodes = VecDeque::from([root.clone()]);
    let mut count = 0;
    if root.is_some() {
        nodes.push_back(None);
    }

    while let Some(front) = nodes.pop_front() {
        if let Some(node_rc) = front {
            let node = node_rc.borrow();
            level.push(node.val);

            if node.left.is_some() {
                nodes.push_back(node.left.clone());
            }
            if node.right.is_some() {
                nodes.push_back(node.right.clone());
            }
        } else {
            if !nodes.is_empty() { nodes.push_back(None); }
            helper(&mut count, level);
            level = vec![];
        }
    }

    count
}

pub fn main() {
    // let root = vec![Some(1), Some(4), Some(3), Some(7), Some(6), Some(8), Some(5), None, None, None, None, Some(9), None, Some(10)];
    let root = vec![Some(1), Some(3), Some(2), Some(7), Some(6), Some(5), Some(4)];
    println!("{}", minimum_operations(vec_to_tree(root)));
}
