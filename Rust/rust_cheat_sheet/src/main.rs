#![allow(unused_variables)]
#![allow(unused_assignments)]
#![allow(dead_code)]

include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "01_hello_world.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "02_variables_and_primitives.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "03_strings_and_print.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "04_vectors_and_tuples.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "05_functions_and_scope.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "06_control_flow_and_loops.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "07_structs.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "08_enums.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "09_traits.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "10_ownership.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "11_borrowing.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "12_generics.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "13_closures.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "14_polars_basics.rs"));
include!(concat!(env!("CARGO_MANIFEST_DIR"), "/src/", "15_polars_advanced.rs"));

macro_rules! presentation {
	($f:expr) => {
		println!("\n------------------- {} -------------------", stringify!($f));
		$f();
	}
}

fn main() {
	presentation!(hello_world);
	presentation!(variables_and_primitives);
	presentation!(strings_and_print);
	presentation!(vectors_and_tuples);
	presentation!(control_flow_and_loops);
	presentation!(structs);
	presentation!(enums);
	presentation!(functions_and_scope);
	presentation!(traits);
	presentation!(ownership);
	presentation!(borrowing);
	presentation!(generics);
	presentation!(closures);
	presentation!(polars_basic);
	presentation!(polars_advanced);
}
