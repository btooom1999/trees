fn dfs(
    visited: &mut Vec<bool>,
    i: usize,
    arr: &Vec<i32>,
) -> bool {
    if visited[i] {
        return false;
    }

    if arr[i] == 0 {
        return true;
    }

    visited[i] = true;
    if let Some(l) = i.checked_sub(arr[i] as usize) && dfs(visited, l, arr) {
        return true;
    }
    if i + (arr[i] as usize) < arr.len() && dfs(visited, i+arr[i] as usize, arr) {
        return true;
    }
    visited[i] = false;

    false
}

fn can_reach(arr: Vec<i32>, start: i32) -> bool {
    dfs(&mut vec![false; arr.len()], start as usize, &arr)
}

pub fn main() {
    let arr = [3,0,2,1,2].to_vec();
    let start = 2;
    println!("{}", can_reach(arr, start));
}
