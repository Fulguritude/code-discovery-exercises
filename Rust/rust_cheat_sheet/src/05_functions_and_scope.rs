// Like many languages, Rust has functions and code-block-based scoping.
// Rust also has nested functions, since functions are prime citizens in Rust.



fn function_with_params_and_return_type(param1: bool, param2: i32) -> i32
{
	// Writing an expression, for example a variable's name, alone,
	// at the end of a scope, will have the scope take the value of
	// this expression as its own value. Here, the function will
	// return the value of param2
	// Note the absence of semicolon.
	param2
}

fn functions_and_scope()
{
	let param1: bool = true;

	let result: i32 = function_with_params_and_return_type(param1, 42);

	// Anonymous scope
	{
		// All variables declared in this scope are only accessible in the scope
		let local_variable = 42;
		// and they are dropped at the end 
	}

	// Every scope can return a value
	let s =
	{
		let ret = "hello";
		ret
	}; // Note the semicolon after the last brace
	println!("s: {}", s);

	// You can define nested functions
	fn nested_function()
	{
		println!("nested function called!");
	}

	nested_function();
}