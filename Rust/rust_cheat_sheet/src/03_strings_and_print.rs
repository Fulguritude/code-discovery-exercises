// Rust has multiple types of strings, which serve various roles. These
// various types were implemented in the standard in order to make safe,
// but performant, string manipulation possible; given the usual memory
// management problems that come with strings.



fn strings_and_print()
{
	// &str is a read-only array of characters
	let string_literal: &str = "text of string literal";

	// String is a structure that allows you to modify a string
	let mut heap_allocated_string: String = String::from("text of heap allocated string");

	// Mutable strings can be append with a `&str` thanks to the `push_str` method
	heap_allocated_string.push_str(string_literal);

	// A String can be composed with format! (think 'sprintf' in C)
	// Each '{}' will be replaced by the corresponding variable after the comma
	// The 'composed_string' String below will contain "Hello text of string literal !"
	let composed_string = format!("Hello {} !", string_literal);

	// Print a string (think 'printf' in C)
	print!("{}", composed_string);
	// Print string, followed by a line return
	println!("{} with line return", composed_string);
}