// implementing a function that returns the kth smallest element in a given array

fn kth_smallest(arr: &mut Vec<i32>, k: usize) -> Option<i32> {
    arr.sort(); // Sort the array in-place
    arr.get(k - 1).cloned()
}

fn main() {
    let mut my_array = vec![9, 1, 5, 3, 20, 25];
    let k = 3;
    if let Some(result) = kth_smallest(&mut my_array, k) {
        println!("K'th smallest element is {}", result);
    } else {
        println!("Invalid input or k out of bounds.");
    }
}