// finding the maximum subarray sum in rust

use max_subarray_sum::Elements;

fn main() {
    let list = vec![-2, -3, 4, -1, -2, 1, 5, -3];
 
    let list = [-2, -3, 4, -1, -2, 1, 5, -3];

    let elements = Elements::new(&mut list); 
    
    let max_sum = elements.find_max_sum().result();

    assert_eq!(7, max_sum);
}