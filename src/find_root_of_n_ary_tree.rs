use std::{cell::RefCell, collections::{HashSet, VecDeque}, rc::Rc};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    children: RefCell<Vec<Rc<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32, children: Option<RefCell<Vec<Rc<TreeNode>>>>) -> Self {
        Self { val, children: children.unwrap_or_default() }
    }
}

fn vec_to_tree(nums: Vec<Option<i32>>) -> Option<Rc<TreeNode>> {
    if nums.is_empty() {
        return None;
    }

    let root = Rc::new(TreeNode::new(nums[0].unwrap(), None));
    let mut nodes = VecDeque::from([root.clone()]);

    let mut i = 1;
    while i < nums.len() && let Some(front) = nodes.pop_front() {
        i += 1;
        let mut node = front.children.borrow_mut();
        while i < nums.len() && let Some(num) = nums[i] {
            let child = Rc::new(TreeNode::new(num, None));
            node.push(child.clone());
            nodes.push_back(child.clone());
            i += 1;
        }
    }

    Some(root)
}

fn find_root(list: Vec<Rc<TreeNode>>) -> Option<Rc<TreeNode>> {
    let mut roots = HashSet::new();
    let mut children = HashSet::new();

    for node in list.iter() {
        roots.insert(node.val);

        for child in node.children.borrow().iter() {
            children.insert(child.val);
        }
    }

    if let Some(root) = roots.into_iter().find(|v| !children.contains(v)) {
        list.into_iter().find(|node| node.val == root)
    } else {
        None
    }
}

pub fn main() {
    let root = vec![Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)];
    let child1 = vec![Some(3), Some(5), Some(6)];
    let child2 = vec![Some(2)];
    let child3 = vec![Some(4)];
    println!("{:#?}", find_root(vec![
        vec_to_tree(root).unwrap(),
        vec_to_tree(child1).unwrap(),
        vec_to_tree(child2).unwrap(),
        vec_to_tree(child3).unwrap(),
    ]));
}
