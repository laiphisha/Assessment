//implementing a function that returns the shortest word in the string

fn shortest_word(s: &str) -> Option<&str> 
{
    s.split_whitespace().min_by_key(|word| word.len())
}

fn main()
 {
    let sentence = "It is a very beautiful day";
    if let Some(shortest) = shortest_word(sentence)
     {
        println!("Shortest word: {}", shortest);
    }
     else
      {
        println!("No words found in the string");
    }
}
