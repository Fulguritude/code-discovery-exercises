// A generic type is an abstract type that can be replaced by one of the concrete types.

// A struct which can contain any concrete type
struct MyBox<T>
{
	content: T,
}

// A function which can take any concrete type
fn func_with_generic_param<T>(param: T) -> MyBox<T>
{
	MyBox{content: param}
}


// You can use a generic type that extends a certain trait. Ie, only a concrete type
// that have implemented this given trait can be used there.
// Eg, the 'logger' function below only takes concrete types that can be converted to string
fn logger<T: ToString>(event: T)
{
	println!("{}", event.to_string());
}

fn generics()
{
	let my_box_with_str = MyBox { content : "Hello" };
	let my_box_with_i32 = MyBox { content :      42 };
	
	let ret = func_with_generic_param("foo");
	logger("barr");
}

// TODO, maybe add arithmetic on generics, since they can get much more complex ?
