// merging 2 sorted arrays in Rust

fn main() {
    let array1 = [1, 2, 3, 4, 5];
    let array2 = [6, 7, 8, 9, 10];

    let merged = [&array1[..], &array2[..]].concat();

    println!("{:?}", merged);
}