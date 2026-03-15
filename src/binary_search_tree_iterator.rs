use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

struct BSTIterator {
    nodes: VecDeque<i32>
}

impl BSTIterator {
    fn dfs(node: Option<Rc<RefCell<TreeNode>>>, nodes: &mut VecDeque<i32>) {
        if let Some(node_rc) = node {
            let node = node_rc.borrow();

            Self::dfs(node.left.clone(), nodes);
            nodes.push_back(node.val);
            Self::dfs(node.right.clone(), nodes);
        }
    }

    fn new(root: Option<Rc<RefCell<TreeNode>>>) -> Self {
        let mut nodes = VecDeque::new();

        Self::dfs(root, &mut nodes);

        Self { nodes }
    }

    fn next(&mut self) -> i32 {
        self.nodes.pop_front().unwrap_or(-1)
    }

    fn has_next(&self) -> bool {
        !self.nodes.is_empty()
    }
}

pub fn main() {
    let mut bst_iterator = BSTIterator::new(vec_to_tree(vec![Some(7), Some(3), Some(15), None, None, Some(9), Some(20)]));
    println!("{:?}", bst_iterator.nodes);
}
