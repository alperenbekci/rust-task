// Define a function to concatenate two string slices
fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new(); // Create a new String
    result.push_str(s1); // Append the first string slice
    result.push_str(s2); // Append the second string slice
    result // Return the result
}

fn main() {
    // Initialize two String variables
    let string1 = String::from("Hello, ");
    let string2 = String::from("world!");

    // Call the concatenate_strings function with references
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the result to the console
    println!("{}", concatenated_string);
}
