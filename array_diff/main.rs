fn array_diff<T: PartialEq>(a: Vec<T>, b: Vec<T>) -> Vec<T> {
    a.into_iter().filter(|x| !b.contains(x)).collect()
}

fn main() {
    let result1 = array_diff(vec![1, 2], vec![1]);
    println!("{:?}", result1); // Output: [2]

    let result2 = array_diff(vec![1, 2, 2, 2, 3], vec![2]);
    println!("{:?}", result2); // Output: [1, 3]
}
