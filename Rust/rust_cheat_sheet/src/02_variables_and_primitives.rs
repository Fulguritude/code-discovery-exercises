// Some examples of variables and primitive types of the Rust language



fn variables_and_primitives()
{
	// Variables are immutable by default (think 'const' in C)
	let a = 42;
	// You need to add the keyword 'mut' to define a variable as mutable
	let mut b = 41;
	b += 1;

	// The variable's type can be infered
	let c = 42;
	// or specified before the value
	let d: i32 = 42;
	// or with a suffix to the value
	let e = 42i32;

	// List of types here: https://doc.rust-lang.org/rust-by-example/primitives.html

	// Constant need to be UPPERCASE, with a specified type
	// and set with a value know at compile time (think '#define' in C)
	const G: f32 = 4.2; 

	// Every type can be heap allocated with Box<T>
	let f: Box<i64> = Box::new(42);

	// Variable shadowing is allowed (creating a new variable with the same name as an other)
	let f: &str = "hello";
	// from here f is a string
}

// Note: constant can be placed in the global scop
const H: &str = "message";
