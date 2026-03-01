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

fn create_binary_tree(descriptions: Vec<Vec<i32>>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut hashmap = HashMap::<i32, Rc<RefCell<TreeNode>>>::new();
    let mut hashset = HashSet::new();

    for desc in descriptions.iter() {
        hashset.insert(desc[0]);
    }

    for desc in descriptions {
        let (parent, child, is_left) = (desc[0], desc[1], desc[2]);
        hashset.remove(&child);

        let parent_node = hashmap
            .entry(parent)
            .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(parent))))
            .clone();

        let child_node = hashmap
            .entry(child)
            .or_insert_with(|| Rc::new(RefCell::new(TreeNode::new(child))))
            .clone();

        let mut node = parent_node.borrow_mut();
        if is_left == 1 {
            node.left = Some(child_node.clone());
        } else {
            node.right = Some(child_node.clone());
        }
    }

    Some(hashmap.remove(hashset.iter().next().unwrap()).unwrap())
}

pub fn main() {
    let descriptions = [[20,15,1].to_vec(),[20,17,0].to_vec(),[50,20,1].to_vec(),[50,80,0].to_vec(),[80,19,1].to_vec()];
    println!("{:?}", create_binary_tree(descriptions.into()));
}
