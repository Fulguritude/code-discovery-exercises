// The `enum` keyword allows the creation of a type which may be one of a few different variants.
// This provides a way to implement type unions; though not are not as strong (yet) as algebraic enums.
// Any variant which is valid as a struct is also valid as an enum.



enum WebEvent
{
	// An `enum` instance may either be `unit-like`,
	PageLoad,
	PageUnload,
	// like tuple structs,
	KeyPress(char),
	Paste(String),
	// or like C-like structures.
	Click { x: i64, y: i64 },
}

// Like structs, enums can have methods
impl WebEvent
{
	// Check the variant and get the inner value if exist.
	fn inspect(&self)
	{
		match self
		{
			WebEvent::PageLoad       => println!("page loaded"),
			WebEvent::PageUnload     => println!("page unloaded"),
			// Destructure `c` from inside the `enum`.
			WebEvent::KeyPress(c)    => println!("pressed '{}'.", c),
			WebEvent::Paste(s)       => println!("pasted \"{}\".", s),
			// Destructure `Click` into `x` and `y`.
			WebEvent::Click { x, y } =>
			{
				println!("clicked at x={}, y={}.", x, y);
			},
		}
	}
}

fn enums()
{
	// Equivalent to C enums
	let page_load = WebEvent::PageLoad;

	// Variant which stores a string
	let paste = WebEvent::Paste("string from clipboard".to_string());

	page_load.inspect();
	paste.inspect();

	// Two commonly used, and important, enums in Rust are 'Option' and 'Result'
	// - Option : https://doc.rust-lang.org/stable/std/option/index.html
	// - Result : https://doc.rust-lang.org/stable/std/result/index.html
}