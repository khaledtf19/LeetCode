fn main() {
    dbg!(rotate(&mut vec![1, 2, 3, 4, 5, 6, 7], 3));
}

pub fn rotate(arr: &mut Vec<i32>, k: i32) -> () {
    let r = k as usize % arr.len();
    for _i in 0..r {
        let el = arr.pop();
        arr.insert(0, el.unwrap());
    }
    dbg!(arr);
}
