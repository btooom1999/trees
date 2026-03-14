use std::{cell::{Ref, RefCell}, collections::{HashMap, VecDeque}, rc::Rc};

#[derive(Debug)]
struct TreeNode {
    val: i32,
    children: RefCell<Vec<Rc<TreeNode>>>,
}

impl TreeNode {
    fn new(val: i32) -> Self {
        Self { val, children: RefCell::new(Vec::new()) }
    }
}

struct LockingTree {
    nodes: HashMap<i32, Rc<TreeNode>>,
    data: HashMap<i32, (i32, i32)>, // (ancestor, locked_by_user)
}

impl LockingTree {
    fn new(parent: Vec<i32>) -> Self {
        let mut nodes: HashMap<i32, Rc<TreeNode>> = HashMap::new();
        let mut data = HashMap::new();

        for i in 1..parent.len() as i32 {
            let ancestor_key = parent[i as usize];
            let ancestor = match nodes.get(&ancestor_key) {
                Some(node) => node.clone(),
                _ => {
                    let node = Rc::new(TreeNode::new(ancestor_key));
                    nodes.insert(ancestor_key, node.clone());
                    data.insert(ancestor_key, (-1, -1));

                    node
                }
            };

            if let Some(data) = data.get_mut(&i) {
                data.0 = ancestor_key;

                let node = nodes.get(&i).unwrap().clone();
                ancestor.children.borrow_mut().push(node);
            } else {
                let node = Rc::new(TreeNode::new(i));
                ancestor.children.borrow_mut().push(node.clone());
                nodes.insert(i, node);
                data.insert(i, (ancestor_key, -1));
            }
        }

        Self { nodes, data }
    }

    fn lock(&mut self, num: i32, user: i32) -> bool {
        let value = self.data.get_mut(&num).unwrap();
        if value.1 != -1 {
            return false;
        }

        value.1 = user;

        true
    }

    fn unlock(&mut self, num: i32, user: i32) -> bool {
        let value = self.data.get_mut(&num).unwrap();
        if value.1 != user {
            return false;
        }

        value.1 = -1;

        true
    }

    fn dfs(node: Rc<TreeNode>, hashmap: &mut HashMap<i32, (i32, i32)>) -> bool {
        let mut res = false;
        for child in node.children.borrow().iter() {
            let flag = Self::dfs(child.clone(), hashmap);
            res = res || flag;

            hashmap.entry(child.val).and_modify(|v| {
                if v.1 != -1 {
                    res = true;
                    v.1 = -1;
                }
            });
        }

        res
    }

    fn upgrade(&mut self, num: i32, user: i32) -> bool {
        let mut ancestor = num;
        while let Some(data) = self.data.get(&ancestor) {
            if data.1 != -1 {
                return false;
            }

            ancestor = data.0;
        }

        let locked_descendant = Self::dfs(
            self.nodes.get(&num).unwrap().clone(),
            &mut self.data
        );

        if !locked_descendant {
            return false;
        }

        self.data.get_mut(&num).unwrap().1 = user;

        true
    }
}

pub fn main() {
    let mut locking_tree = LockingTree::new([-1,0,8,0,7,4,2,3,3,1].into());
    // let mut locking_tree = LockingTree::new([-1,0,0,1,1,2,2].into());
    println!("{}", locking_tree.upgrade(8, 39));
    println!("{}", locking_tree.upgrade(5, 28));
    println!("{}", locking_tree.upgrade(6, 33));
    println!("{}", locking_tree.upgrade(9, 24));
    println!("{}", locking_tree.lock(5, 22));
    println!("{}", locking_tree.upgrade(1, 3));
    println!("{}", locking_tree.lock(5, 20));
    println!("{}", locking_tree.upgrade(0, 38));
    println!("{}", locking_tree.lock(5, 14));
    println!("{}", locking_tree.lock(6, 34));
    println!("{}", locking_tree.lock(6, 28));
    println!("{}", locking_tree.upgrade(3, 23));
    println!("{}", locking_tree.upgrade(4, 45));
    println!("{:?}", locking_tree.data);
}
