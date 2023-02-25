// One of the fundamental design constructs of Rust is ownership
// Ownership allows the compiler to know when memory should be created,
// and when memory should be released. It is the "owner" of a variable
// which releases the memory for that variable, once it is done using it.



fn func1(param1: i32)
{
	// 'param1' is a copy of 'var_with_copy_trait'
}

fn func2(param2: String)
{
	// 'param2' is the original of 'var_without_copy_trait'
}

fn ownership()
{
	// If the variable's type has implemented the 'Copy' trait...
	let var_with_copy_trait = 42;
	// ... we can use this variable as a parameter, which will copy it to a new variable (like in C)...
	func1(var_with_copy_trait);
	// ... and the original 'var_with_copy_trait' is still accessible.
	println!("{}", var_with_copy_trait);

	// Here is a list of standard types which have implemented the 'Copy' trait:
	// https://doc.rust-lang.org/std/marker/trait.Copy.html#implementors

	// If the variable's type has not implemented the 'Copy' trait...
	let var_without_copy_trait = String::from("Hello");
	// ... using this variable as a parameter will "move" it, giving its ownership away ...
	func2(var_without_copy_trait);
	// ... and 'var_without_copy_trait' is not accessible from here anymore.
}