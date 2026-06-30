use std::{cell::{Ref, RefCell}, rc::Rc};

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

fn bst_from_preorder(preorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    let mut root = Rc::new(RefCell::new(TreeNode::new(preorder[0])));
    let mut stack = Vec::from([root.clone()]);

    for num in preorder.into_iter().skip(1) {
        let mut prev = None;
        while stack.last().is_some_and(|node_rc| node_rc.borrow().val < num) {
            prev = stack.pop();
        }

        let node = Rc::new(RefCell::new(TreeNode::new(num)));
        if let Some(prev_rc) = prev {
            prev_rc.borrow_mut().right = Some(node.clone());
        } else {
            stack.last_mut().unwrap().borrow_mut().left = Some(node.clone());
        }
        stack.push(node.clone());
    }

    Some(root)
}

pub fn main() {
    // let preorder = [8,5,1,7,10,12].to_vec();
    let preorder = [3,1,2].to_vec();
    println!("{:?}", bst_from_preorder(preorder));
}
