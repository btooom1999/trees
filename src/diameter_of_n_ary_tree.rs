use std::{cell::RefCell, collections::VecDeque, rc::Rc};

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

        while i < nums.len() && let Some(val) = nums[i] {
            let child = Rc::new(TreeNode::new(val, None));
            node.push(child.clone());
            nodes.push_back(child.clone());
            i += 1;
        }
    }

    Some(root)
}

fn helper(
    node: Option<Rc<TreeNode>>,
    max: &mut i32,
) -> i32 {
    if let Some(node) = node {
        let mut temp_max = (0, 0);
        for child in node.children.borrow().iter() {
            let val = helper(Some(child.clone()), max);
            if val >= temp_max.1 {
                temp_max.0 = temp_max.1;
                temp_max.1 = val;
            } else if val > temp_max.0 {
                temp_max.0 = val;
            }
        }

        *max = std::cmp::max(*max, temp_max.0 + temp_max.1);

        temp_max.1 + 1
    } else {
        0
    }
}

fn diameter(root: Option<Rc<TreeNode>>) -> i32 {
    let mut max = 0;
    helper(root, &mut max);

    max
}

pub fn main() {
    // let root = vec![Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)];
    // let root = vec![Some(1), None, Some(3), Some(2), Some(4), None, Some(5), Some(6)];
    // let root = vec![
    //     Some(1), None, Some(2), Some(3), Some(4), Some(5), None, None,
    //     Some(6), Some(7), None, Some(8), None, Some(9), Some(10), None, None,
    //     Some(11), None, Some(12), None, Some(13), None, None, Some(14)
    // ];
    let root = vec![Some(1), None, Some(2), None, Some(3), Some(4), None, Some(5), None, Some(6)];
    println!("{}", diameter(vec_to_tree(root)));
}
