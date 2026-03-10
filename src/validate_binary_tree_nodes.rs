use std::{cell::RefCell, collections::{HashMap, HashSet, VecDeque}, rc::Rc};

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

fn validate_binary_tree_nodes(n: i32, left_child: Vec<i32>, right_child: Vec<i32>) -> bool {
    let n = n as _;
    let mut hashmap = vec![None; n];
    let mut root = n;

    let mut i = 0;
    while i < n {
        let left = left_child[i];
        let right = right_child[i];

        if i as i32 == left || i as i32 == right {
            return false;
        }

        if left != -1 {
            let left = left as usize;
            if hashmap[left].is_some() {
                return false;
            }

            let mut k = i;
            while let Some(val) = hashmap[k] {
                k = val;
                if val == left {
                    return false;
                }
            }

            root -= 1;
            hashmap[left] = Some(i);
        }

        if right != -1 {
            let right = right as usize;
            if hashmap[right].is_some() {
                return false;
            }

            let mut k = i;
            while let Some(val) = hashmap[k] {
                k = val;
                if val == right {
                    return false;
                }
            }

            root -= 1;
            hashmap[right] = Some(i);
        }

        i += 1;
    }

    root == 1
}

pub fn main() {
    let n = 5;
    let left_child = vec![0,-1,3,1,3];
    let right_child = vec![4,3,0,1,-1];
    println!("{}", validate_binary_tree_nodes(n, left_child, right_child));
}
