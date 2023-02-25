// Closures are nested functions, written with the Rust lambda expression syntax,
// that capture variables from inside their parent function for their own use.
// Given the importance of memory ownership in Rust, these need to be handle specifically,
// since they share memory with their parent.



fn closures()
{
	let mut local_var1 = 41;
	let     local_var2 = "local_var2".to_string();
	let mut local_var3 = "local_var3".to_string();
	let mut local_var4 = "local_var4".to_string();
	let     bool_param = true;

	// Closures are anonymous functions that can capture the enclosing environment
	let mut closure = |bool_param: bool| -> i32
	{
		// Note the use of 'local_var1' although it is not sent as a parameter
		local_var1 += 1;
		println!("{}", local_var1);
		local_var1
	};

	// Closures are called like regular functions
	let result = closure(bool_param);



	// There is 3 differents types of closure:
	// - Fn: the closure uses the captured value by reference (&T)
	// - FnMut: the closure uses the captured value by mutable reference (&mut T)
	// - FnOnce: the closure uses the captured value by value (T) and consume it.

	let closure_fn = |bool_param: bool| -> i32
	{
		println!("{}", &local_var2);
		42
	};

	let closure_fn_mut = |bool_param: bool| -> i32
	{
		let ref_local_var3 = &mut local_var3;
		ref_local_var3.push_str(" modified");
		println!("{}", local_var3);
		42
	};

	let closure_fn_once = |bool_param: bool| -> i32
	{
		local_var4.push_str(" modified");
		// Macro 'dbg!' consumes 'local_var4' and pretty-prints it.
		dbg!(local_var4);
		42
	};
	// From here on, local_var4 is not available anymore, precisely because FnOnce
	// captures at closure declaration, not at closure call


	function_with_fn_closure_in_param(closure_fn, bool_param);
	// local_var2 still available
	println!("{}", local_var2);

	function_with_fn_mut_closure_in_param(closure_fn_mut, bool_param);
	// local_var3 has been modified and still available
	println!("{}", local_var3);

	function_with_fn_once_closure_in_param(closure_fn_once, bool_param);
}


fn function_with_fn_closure_in_param<F: Fn(bool) -> i32>(closure: F, bool_param: bool) {
	closure(bool_param);
}
fn function_with_fn_mut_closure_in_param<F: FnMut(bool) -> i32>(mut closure: F, bool_param: bool) {
	closure(bool_param);
}
fn function_with_fn_once_closure_in_param<F: FnOnce(bool) -> i32>(closure: F, bool_param: bool) {
	closure(bool_param);
}

// |                 | Fn | FnMut | FnOnce |
// |-----------------|----|-------|--------|
// | closure_fn      | ✅ |   ✅   |   ✅   |
// | closure_fn_mut  | ❌ |   ✅   |   ✅   |
// | closure_fn_once | ❌ |   ❌   |   ✅   |