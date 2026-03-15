fn dfs(
    set: &Vec<Vec<i32>>,
    inform_time: &Vec<i32>,
    children: &Vec<i32>,
    time: i32,
    total_time: i32,
    res: &mut i32,
) {
    for &child in children {
         dfs(set, inform_time, &set[child as usize], inform_time[child as usize], total_time + time, res);
    }

    *res = std::cmp::max(*res, total_time);
}

fn num_of_minutes(n: i32, head_id: i32, manager: Vec<i32>, inform_time: Vec<i32>) -> i32 {
    let mut set = vec![vec![]; n as usize];
    let mut leaf = vec![true; n as usize];

    for (child, &parent) in manager.iter().enumerate() {
        if parent != -1 {
            set[parent as usize].push(child as i32);
        }
    }

    let mut res = 0;
    dfs(&set, &inform_time, &set[head_id as usize], inform_time[head_id as usize], 0, &mut res);

    res
}

pub fn main() {
    let n = 11;
    let head_id = 4;
    let manager = [5,9,6,10,-1,8,9,1,9,3,4];
    let inform_time = [0,213,0,253,686,170,975,0,261,309,337];
    println!("{:?}", num_of_minutes(n, head_id, manager.into(), inform_time.into()));
}
