use std::{cell::RefCell, collections::{HashMap, VecDeque}, rc::Rc};

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

fn recover_from_preorder(traversal: String) -> Option<Rc<RefCell<TreeNode>>> {
    let mut hashmap = HashMap::new();
    let mut traversal = VecDeque::from(traversal.into_bytes());

    loop {
        if traversal.is_empty() {
            return Some(hashmap.remove(&1).unwrap());
        }

        let mut level = 0;
        while let Some(&front) = traversal.front() && front == b'-' {
            level += 1;
            traversal.pop_front();
        }

        let mut num = 0_i32;
        while let Some(&front) = traversal.front() && front != b'-' {
            num *= 10;
            num += (front - b'0') as i32;
            traversal.pop_front();
        }

        let node = Rc::new(RefCell::new(TreeNode::new(num)));
        hashmap.insert(level+1, node.clone());
        if level > 0 {
            let mut parent = hashmap.get(&level).unwrap().borrow_mut();
            if parent.left.is_none() { parent.left = Some(node) } else { parent.right = Some(node) }
        }
    }

    None
}

pub fn main() {
    let traversal = "1-2--3--4-5--6--7".to_string();
    println!("{:?}", recover_from_preorder(traversal));
}
