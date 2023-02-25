// There are three types of structures ("structs") that can be created using the struct keyword:
//   - Tuple structs, which are, basically, named tuples.
//   - The classic C structs (ie, fixed-key, statically-defined data dictionaries).
//   - Unit structs, which are field-less, and are useful for generics.



// A unit struct
struct Unit;

// A tuple struct
struct Pair(i32, f32);

// a struct with 2 named fields
struct Person
{
	age  : u32,
	name : String,
}

// For OOP-like design, you can "implement" methods for your structs
impl Person
{
	// Here 'Self' is the type 'Person'
	fn born(name: &str) -> Self
	{
		let name = name.to_string();
		Self { age: 0, name }
	}

	// The variable 'self' is the current instance of 'Person' (equal to 'this' in other langage)
	fn birthday(&mut self)
	{
		self.age += 1;
	}
}

fn structs()
{
	let pair = Pair(-42, 42.42);

	let mut person = Person::born("John");
	println!("{} is {}", person.name, person.age);
	person.birthday();
	println!("{} is {}", person.name, person.age);

	let person_twin_name = "Bill".to_string();
	// Create a new struct with content that is partially the same
	let person_twin = Person { name: person_twin_name, ..person };

	println!("{} is {} too", person_twin.name, person_twin.age);
}


