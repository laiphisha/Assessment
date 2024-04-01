//implementing a function that checks whether a given string is a palindrome or not

fn is_palindrome(s: &str) -> bool {
    // Convert the string to lowercase
    let s_lower = s.to_lowercase();

    // Check if the reversed string is equal to the original string
    s_lower.chars().eq(s_lower.chars().rev())
}

fn main() {
    let string1 = "madam"; // Define a palindrome string
    let string2 = "hello"; // Define a non-palindrome string

    // Check if string1 is a palindrome
    if is_palindrome(string1) {
        println!("'{}' is a palindrome.", string1);
    } else {
        println!("'{}' is not a palindrome.", string1);
    }

    // Check if string2 is a palindrome
    if is_palindrome(string2) {
        println!("'{}' is a palindrome.", string2);
    } else {
        println!("'{}' is not a palindrome.", string2);
    }
}