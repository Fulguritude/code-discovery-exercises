// Standard keywords of Turing-complete languages, and their use in Rust:
// if, else, match, for, while, loop



fn control_flow_and_loops()
{
	let mut i = 41;
	// Conditions need to be strictly of boolean type, unlike in C
	// Braces are mandatory even if there is only one line in the block
	if i > 0
	{
		i += 1;
	}
	else if i == 0
	{
		i += 2;
	}
	else
	{
		i *= 3;
	}

	let condition = true;
	// Conditional blocks have scope, so they can return a value via an expression
	// Each branch needs to return the same type
	// All cases need to be covered
	let return_value =
	if condition
	{
		"condition is true"
	}
	else
	{
		"condition is false"
	}; // Note the semicolon after the last brace
	println!("{}", return_value);


	// Pattern matching can be used like 'switch-case' statements in C
	// All cases need to be covered
	match i
	{
		0  => println!("i has value zero"),
		1 => println!("i has value one"),
		_ => println!("i has an other value"),
	}

	// Like if-blocks, match-blocks can also return value via an expression
	let return_value = match condition
	{
		true  => "condition is true",
		false => "condition is false",
	};
	println!("{}", return_value);



	// while-loops function like in C
	let mut i = 0;
	while i < 10
	{
		print!("{} ", i);
		i += 1;
	}
	println!("");

	// Example of a for-loop over a range, here, it prints 0 to 9 included
	for i in 0..10
	{
		print!("{} ", i);
	}
	println!("");

	// `loop` is used for infinite loops (aka `while(True)` loops)
	// Loops can be named with a label
	'parent_loop: loop
	{
		'_child_loop: loop
		{
			if i >= 0
			{
				// Use the label to specify the loop to break out of
				break 'parent_loop;
			}

		}
	}
}