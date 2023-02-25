// Sometimes we want to use a variable created in a parent function in a child function,
// but for the parent to re-obtain the variable after the child function has completed.
// For this, the child function can "borrow" the variable, ie, own it temporarily.
// This is done via the use of the & symbol.



fn function_with_ref(param1: &String)
{
	// 'param1' is a reference of 'local_var', and can be read here
	println!("print from function_with_ref: {}", param1);
}

fn function_with_mut_ref(param2: &mut String)
{
	// 'param2' is mutable, so can even be modified/written to here
	param2.push_str("appended message");
}

fn borrowing()
{
	let local_var = String::from("Hello");
	// Borrow this variable by sending its reference.
	function_with_ref(&local_var);
	// 'local_var' is still accessible here
	println!("{}", local_var);


	let mut mut_local_var = String::from("World");
	// You can lend this variable by using its mutable reference
	function_with_mut_ref(&mut mut_local_var);
	// 'mut_local_var' is now modified
	println!("{}", mut_local_var);

	// Note: you can't create multiple mutable references,
	// or mutable and immutable reference to the same variable (for memory safety reason).
}