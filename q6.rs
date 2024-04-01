/*
implementing a function that finds the longest common prefix of a qiven set of strings
*/

fn longest_common_prefix(strings: &[&str]) -> String {
    if strings.is_empty() 
    {
        return String::new(); // If the input is empty, return an empty string
    }

    let first_str = strings[0]; // Get the first string in the array
    let mut prefix = String::new(); // Initialize the prefix string

    'outer: for (i, c) in first_str.char_indices() {
        for s in &strings[1..] {
            if let Some(sc) = s.chars().nth(i) {
                if sc != c {
                    break 'outer; // If characters don't match, break the loop
                }
            } else {
                break 'outer; // If the string is shorter than expected, break the loop
            }
        }
        prefix.push(c); // Add character to the prefix string
    }

    prefix // Return the longest common prefix
}

fn main() {
    let strings = ["flower", "flow", "flight"];
    let prefix = longest_common_prefix(&strings);
    println!("Longest common prefix: {}", prefix);
}