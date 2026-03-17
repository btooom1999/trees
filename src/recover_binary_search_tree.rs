use std::{cell::RefCell, rc::Rc};

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

// fn helper(node: Option<Rc<RefCell<TreeNode>>>, vec: &mut Vec<Rc<RefCell<TreeNode>>>) {
//     if let Some(node_rc) = node {
//         let node = node_rc.borrow();

//         helper(node.left.clone(), vec);
//         vec.push(node_rc.clone());
//         helper(node.right.clone(), vec);
//     }
// }

// fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
//     let mut vec = Vec::new();

//     helper(root.clone(), &mut vec);

//     let (mut node1, mut node2) = (None, None);
//     for i in 0..vec.len()-1 {
//         if vec[i].borrow().val > vec[i+1].borrow().val {
//             node2 = Some(vec[i+1].clone());
//             if node1.is_none() {
//                 node1 = Some(vec[i].clone());
//             }
//         }
//     }

//     if let (Some(node1), Some(node2)) = (node1, node2) {
//         let val1 = node1.borrow().val;
//         let val2 = node2.borrow().val;
//         node1.borrow_mut().val = val2;
//         node2.borrow_mut().val = val1;
//     }
// }

fn recover_tree(root: &mut Option<Rc<RefCell<TreeNode>>>) {
    let mut stack = Vec::new();
    let mut curr = root.clone();
    let mut prev = None as Option<Rc<RefCell<TreeNode>>>;
    let (mut node1, mut node2) = (None, None);
    while !stack.is_empty() || curr.is_some() {
        while let Some(node_rc) = curr {
            stack.push(node_rc.clone());
            curr = node_rc.borrow().left.clone();
        }

        if let Some(popped) = stack.pop() {
            if let Some(prev) = prev.clone() && prev.borrow().val > popped.borrow().val {
                node2 = Some(popped.clone());
                if node1.is_none() {
                    node1 = Some(prev.clone());
                }
            }

            prev = Some(popped.clone());
            curr = popped.borrow().right.clone();
        }
    }

    if let (Some(node1), Some(node2)) = (node1, node2) {
        let mut node1 = node1.borrow_mut();
        let mut node2 = node2.borrow_mut();
        (node1.val, node2.val) = (node2.val, node1.val);
    }
}

pub fn main() {
    // let mut root = vec_to_tree(vec![Some(1), Some(3), None, None, Some(2)]);
    let mut root = vec_to_tree(vec![Some(3), Some(1), Some(4), None, None, Some(2)]);
    recover_tree(&mut root);
    println!("{:#?}", root);
}


