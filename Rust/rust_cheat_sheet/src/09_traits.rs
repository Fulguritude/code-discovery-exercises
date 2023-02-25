// A trait is a bunch of methods/const values that are to be implemented for a given type.
// You can think of traits as mixins or interfaces in C++



// The easy way to implement a trait is to use a `derive` macro
#[derive(Debug, PartialEq, Eq)]
struct Cat
{
	age  : u32,
	name : String,
}
// Here is the list of `derive` macros of standard Rust:
// https://doc.rust-lang.org/book/appendix-03-derivable-traits.html



// Manual trait creation
trait Characteristics
{
	// Method signatures
	fn get_age  (&self) -> u32;
	fn get_name (&self) -> String;
	fn re_name  (&mut self, new_name: &str);

	// Traits can provide default method definitions.
	fn description(&self)
	{
		println!("age: {}, name: {}", self.get_age(), self.get_name());
	}
}

// Manual trait implementation
impl Characteristics for Cat
{
	fn get_age(&self) -> u32
	{
		return self.age;
	}
	fn get_name(&self) -> String
	{
		return self.name.clone();
	}
	fn re_name(&mut self, new_name: &str)
	{
		self.name = new_name.to_string();
	}
}


fn traits()
{
	let cat = Cat { age: 5, name: "Sylvester".to_string() };

	// Thanks to 'Debug' trait, the content of cat can be printed prettily (like console.log())
	println!("{:?}", cat);

	let another_cat = Cat { age: 5, name: "Garfield".to_string() };
	// Thanks to 'PartialEq' and 'Eq', two cats can be compared
	if cat != another_cat
	{
		println!("These 2 cats are not the same")
	}

	cat.description();
}