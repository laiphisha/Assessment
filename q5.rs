//implementing a function that returns the median of the array

fn find_median(sorted_array: &[i32]) -> f64
 {
    let len = sorted_array.len();
    let mid = len / 2;

    if len % 2 == 0 {
        // Even-length array
        let left = sorted_array[mid - 1];
        let right = sorted_array[mid];
        f64::from(left + right) / 2.0
    } else {
        // Odd-length array
        f64::from(sorted_array[mid])
    }
}

fn main()
 {
    let sorted_array = vec![1, 2, 3, 4, 5, 6,7,8];
    let median = find_median(&sorted_array);
    println!("Median: {}", median);
}