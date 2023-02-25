// There are 3 main types of data lists in Rust:
//    - tuples,
//    - arrays (stack-allocated lists of a single type)
//    - vecs (heap-allocated arrays)



fn vectors_and_tuples()
{
	// Stack allocated arrays are defined as [T; length]
	let stack_allocated_array: [i32; 3] = [1, 2, 3];

	// All elements can be initialized to the same value
	let array_of_same_value: [i32; 500] = [0; 500];

	// One can also allocate arrays on the heap, called `Vec`s
	let mut heap_allocated_array: Vec<i32> = vec![1, 15, 42];

	// Access to one element is done with an [index] suffix, like in C
	heap_allocated_array[0] = 2;

	// You can iterate over an array with the 'for-in' syntax
	for case in stack_allocated_array
	{
		println!("case: {}", case);
	}
	// or with the iterator
	heap_allocated_array
		.iter()
		.for_each(
			|case|
			{
				println!("case: {}", case);
			}
		);

	// Tuples are collections of values of different types
	let tuple = ("a string", 42);

	// Access to one element is done with .index
	println!("the first element is: {}", tuple.0);

	// Tuples can be destructured
	let (a, b) = tuple;
	println!("a: {}, b: {}", a, b);
}