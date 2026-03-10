fn verify_preorder(preorder: Vec<i32>) -> bool {
    let mut stack = Vec::new();
    let mut root = i32::MIN;
    for &num in &preorder {
        if root > num {
            return false;
        }

        while let Some(&old_num) = stack.last() && old_num < num {
            root = stack.pop().unwrap();
        }

        stack.push(num);
    }

    true
}

pub fn main() {
    let preorder = [5,2,6,1,3];
    println!("{}", verify_preorder(preorder.into()));
}
