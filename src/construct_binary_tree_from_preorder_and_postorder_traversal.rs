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

fn helper(preorder: Vec<i32>, postorder: &mut Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    if preorder.is_empty() {
        return None;
    }

    let val = postorder.pop().unwrap();
    let root = Rc::new(RefCell::new(TreeNode::new(val)));

    if let Some(right) = postorder.last() && let Some(idx) = preorder.iter().position(|v| v == right) {
        root.borrow_mut().right = helper(preorder[idx..].into(), postorder);
    }

    if let Some(left) = postorder.last() && let Some(idx) = preorder.iter().position(|v| v == left) {
        root.borrow_mut().left = helper(preorder[idx..].into(), postorder);
    }

    Some(root)
}

fn construct_from_pre_post(preorder: Vec<i32>, mut postorder: Vec<i32>) -> Option<Rc<RefCell<TreeNode>>> {
    helper(preorder, &mut postorder)
}

pub fn main() {
    let preorder = [1,2,4,5,3,6,7];
    let postorder = [4,5,2,6,7,3,1];
    println!("{:#?}", construct_from_pre_post(preorder.into(), postorder.into()));
}
