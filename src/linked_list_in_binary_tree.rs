use std::{cell::RefCell, collections::VecDeque, rc::Rc};

struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

impl ListNode {
    fn new(val: i32) -> Self {
        Self { val, next: None }
    }
}

fn vec_to_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;
    let mut tail = &mut head;

    for num in nums {
        *tail = Some(Box::new(ListNode::new(num)));
        tail = &mut tail.as_mut().unwrap().next;
    }

    head
}

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

        nodes = new_nodes
    }

    Some(root)
}

#[inline]
fn convert_linked_list_to_vec(mut head: Option<Box<ListNode>>) -> VecDeque<i32> {
    let mut vec = VecDeque::new();
    while let Some(node) = head {
        head = node.next;
        vec.push_back(node.val);
    }

    vec
}

fn helper(node: Option<Rc<RefCell<TreeNode>>>, mut vec: VecDeque<i32>, condition: VecDeque<i32>, contained: &mut bool) {
    if let Some(node_rc) = node {
        if *contained {
            return;
        }

        let node = node_rc.borrow();
        vec.push_back(node.val);
        if vec.len() > condition.len() {
            vec.pop_front();
        }

        if vec == condition {
            *contained = true;
            return;
        }

        helper(node.left.clone(), vec.clone(), condition.clone(), contained);
        helper(node.right.clone(), vec.clone(), condition.clone(), contained);
    }
}

fn is_sub_path(head: Option<Box<ListNode>>, root: Option<Rc<RefCell<TreeNode>>>) -> bool {
    let condition = convert_linked_list_to_vec(head);
    let mut contained = false;
    helper(root, VecDeque::new(), condition, &mut contained);

    contained
}

pub fn main() {
    let root = vec![Some(1), Some(4), Some(4), None, Some(2), Some(2), None, Some(1), None, Some(6), Some(8), None, None, None, None, Some(1), Some(3)];
    let head = [4,2,8];
    println!("{}", is_sub_path(vec_to_list(head.into()), vec_to_tree(root)));
}
