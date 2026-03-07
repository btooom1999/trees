use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug)]
struct Node {
    val: bool,
    is_leaf: bool,
    top_left: Option<Rc<RefCell<Node>>>,
    top_right: Option<Rc<RefCell<Node>>>,
    bottom_left: Option<Rc<RefCell<Node>>>,
    bottom_right: Option<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(val: bool, is_leaf: bool) -> Self {
        Self { val, is_leaf, top_left: None, top_right: None, bottom_left: None, bottom_right: None }
    }
}

fn helper(grid: Vec<Vec<i32>>, is_leaf: bool) -> Option<Rc<RefCell<Node>>> {
    if grid.is_empty() {
        return Some(Rc::new(RefCell::new(Node::new(false, true))));
    }

    if is_leaf || grid[0].len() == 1 {
        return Some(Rc::new(RefCell::new(Node::new(grid[0][0] == 1, true))));
    }

    let amount = grid[0].len();

    let mut is_leaf = [true, true];
    let mut quadra_is_leaf = HashMap::new();
    quadra_is_leaf.insert((0,0), [true, true]);
    quadra_is_leaf.insert((0,1), [true, true]);
    quadra_is_leaf.insert((1,0), [true, true]);
    quadra_is_leaf.insert((1,1), [true, true]);

    for (i, row) in grid.iter().enumerate() {
        for (j, &num) in row.iter().enumerate() {
            is_leaf[num as usize] = false;

            let i = i / (amount/2);
            let j = j / (amount/2);
            quadra_is_leaf.entry((i, j)).and_modify(|is_leaf| is_leaf[num as usize] = false);
        }
    }

    if !is_leaf[0] && !is_leaf[1] {
        let node_rc = Rc::new(RefCell::new(Node::new(false, false)));
        let mut node = node_rc.borrow_mut();
        let grid_top_left = grid[..amount/2].iter().map(|row| row[..amount/2].to_vec()).collect::<Vec<_>>();
        let grid_top_right = grid[..amount/2].iter().map(|row| row[amount/2..].to_vec()).collect::<Vec<_>>();
        let grid_bottom_left = grid[amount/2..].iter().map(|row| row[..amount/2].to_vec()).collect::<Vec<_>>();
        let grid_bottom_right = grid[amount/2..].iter().map(|row| row[amount/2..].to_vec()).collect::<Vec<_>>();
        node.top_left = helper(grid_top_left, quadra_is_leaf.get(&(0,0)).unwrap().iter().any(|v| *v));
        node.top_right = helper(grid_top_right, quadra_is_leaf.get(&(0,1)).unwrap().iter().any(|v| *v));
        node.bottom_left = helper(grid_bottom_left, quadra_is_leaf.get(&(1,0)).unwrap().iter().any(|v| *v));
        node.bottom_right = helper(grid_bottom_right, quadra_is_leaf.get(&(1,1)).unwrap().iter().any(|v| *v));

        return Some(node_rc.clone());
    }

    Some(Rc::new(RefCell::new(Node::new(grid[0][0] == 1, true))))
}

fn construct(grid: Vec<Vec<i32>>) -> Option<Rc<RefCell<Node>>> {
    helper(grid, false)
}

pub fn main() {
    let grid = vec![
        vec![0,1],
        vec![1,0]
    ];
    // let grid = [
    //     [1,1,1,1,0,0,0,0],
    //     [1,1,1,1,0,0,0,0],
    //     [1,1,1,1,1,1,1,1],
    //     [1,1,1,1,1,1,1,1],
    //     [1,1,1,1,0,0,0,0],
    //     [1,1,1,1,0,0,0,0],
    //     [1,1,1,1,0,0,0,0],
    //     [1,1,1,1,0,0,0,0]
    // ].into_iter().map(|v| v.to_vec()).collect::<Vec<_>>();
    // println!("{:#?}", construct(grid));
    construct(grid);
}
