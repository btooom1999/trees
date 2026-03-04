use std::{cell::RefCell, collections::HashMap, rc::Rc};

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
    triple_hashmap: &mut HashMap<(i32, i32, i32), i32>,
    hashmap: &mut HashMap<i32, i32>,
    res: &mut Vec<Option<Rc<RefCell<TreeNode>>>>,
    next_id: &mut i32,
) -> i32 {
    if let Some(node_rc) = node {
        let node = node_rc.borrow();
        let left_id = helper(node.left.clone(), triple_hashmap, hashmap, res, next_id);
        let right_id = helper(node.right.clone(), triple_hashmap, hashmap, res, next_id);

        let key = (node.val, left_id, right_id);
        let id = match triple_hashmap.get(&key) {
            Some(&id) => id,
            _ => {
                let id = *next_id;
                *next_id += 1;
                triple_hashmap.insert(key, id);
                id
            }
        };

        let val = hashmap.entry(id).and_modify(|v| *v += 1).or_insert(1);
        if *val == 2 {
            res.push(Some(node_rc.clone()));
        }

        id
    } else {
        0
    }
}

fn find_duplicate_subtrees(root: Option<Rc<RefCell<TreeNode>>>) -> Vec<Option<Rc<RefCell<TreeNode>>>> {
    let mut triple_hashmap = HashMap::new();
    let mut hashmap = HashMap::new();
    let mut res = Vec::new();
    helper(root, &mut triple_hashmap, &mut hashmap, &mut res, &mut 1);

    res
}

pub fn main() {
    let root = vec![Some(1), Some(2), Some(3), Some(4), None, Some(2), Some(4), None, None, Some(4)];
    println!("{:?}", find_duplicate_subtrees(vec_to_tree(root)));
}
