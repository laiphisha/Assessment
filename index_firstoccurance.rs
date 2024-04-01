//implementing a function that return the indeex of the first occurance of a given number

fn find_first_occurrence(arr: &[i32], target: i32) -> isize {
    let mut first_index = -1;
    for (i, &num) in arr.iter().enumerate() {
        if num == target {
            first_index = i as isize;
            break;
        }
    }
    first_index
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8, 9, 9];
    let target = 2;
    let first_occurrence = find_first_occurrence(&arr, target);

    if first_occurrence != -1 
    {
        println!("First Occurrence = {}", first_occurrence);
    } 
    else
     {
        println!("Not Found");
    }
}