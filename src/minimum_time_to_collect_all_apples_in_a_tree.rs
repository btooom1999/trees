use std::collections::HashMap;

fn dfs(curr: usize, parent: usize, hashmap: &HashMap<usize, Vec<usize>>, has_apple: &Vec<bool>) -> i32 {
    let mut total_time = 0;

    if let Some(children) = hashmap.get(&curr) {
        for &child in children {
            if child == parent { continue; }

            let child_time = dfs(child, curr, hashmap, has_apple);
            if child_time > 0 || has_apple[child] {
                total_time += child_time + 2;
            }
        }
    }

    total_time
}

fn min_time(n: i32, mut edges: Vec<Vec<i32>>, mut has_apple: Vec<bool>) -> i32 {
    let mut hashmap = HashMap::<usize, Vec<usize>>::new();

    for edge in &edges {
        hashmap.entry(edge[0] as usize).or_default().push(edge[1] as usize);
        hashmap.entry(edge[1] as usize).or_default().push(edge[0] as usize);
    }

    let mut time = 0;

    dfs(0, usize::MAX, &hashmap, &has_apple)
}

pub fn main() {
    let n = 7;
    let edges = [[0,1],[0,2],[1,4],[1,5],[2,3],[2,6]].into_iter().map(Vec::from).collect::<Vec<_>>();
    let has_apple = [false,false,true,false,true,true,false].to_vec();
    // let n = 4;
    // let edges = [[0,1], [1,2], [0,3]].into_iter().map(Vec::from).collect::<Vec<_>>();
    // let has_apple = [true, true, true, true].to_vec();
    println!("{}", min_time(n, edges, has_apple));
}
