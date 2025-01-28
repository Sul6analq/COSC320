// TODO: Fix the compiler error in this function.
fn fill_vec(vec: Vec<i32>) -> Vec<i32> {
    let mut vec = vec;

    vec.push(88);
    println!("{:?}",vec);
    vec
}

fn main() {
    // You can optionally experiment here.
    let vec: Vec<i32> = vec![1,2,3];
    println!("{:?}",vec);
    fill_vec(vec);
    
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn move_semantics1() {
        let vec0 = vec![22, 44, 66];
        let vec1 = fill_vec(vec0);
        assert_eq!(vec1, vec![22, 44, 66, 88]);
    }
}