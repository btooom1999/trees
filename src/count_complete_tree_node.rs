use std::rc::Rc;
use std::cell::RefCell;

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

fn count_nodes(root: Option<Rc<RefCell<TreeNode>>>) -> i32 {
    if root.is_none() {
        return 0;
    }

    let mut depth = 0;
    let mut minus = 0;
    let mut queue = vec![(root.unwrap(), 1)];
    while let Some((node_rc, d)) = queue.pop() {
        let node = node_rc.borrow();
        if node.left.is_none() {
            if d > depth && minus > 0 {
                return (1 << (d)) - 1 - minus;
            }
            minus += 2;
        } else if node.right.is_none() {
            return (1 << (d+1)) - 2 - minus;
        } else {
            queue.push((node.left.clone().unwrap(), d+1));
            queue.push((node.right.clone().unwrap(), d+1));
        }

        depth = depth.max(d);
    }

    (1 << depth) - 1
}

pub fn main() {
    let root = vec![Some(1), Some(2), Some(3), Some(4), Some(5)];
    // let root = vec![Some(1)];
    println!("{}", count_nodes(vec_to_nums(root)));
}
