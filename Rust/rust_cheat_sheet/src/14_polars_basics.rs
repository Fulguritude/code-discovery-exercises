// Polars is pretty much the standard Rust DataFrame (spreadsheet/relational table)
// library. It's a faster equivalent to Python's "pandas".



use polars::prelude::*;

// We return a Result to be able to use the `?` operator. 
// Even if the failure case is not handled in main, at least this file is more readable
fn polars_basic() -> Result<()> 
{
	// Polars cheatsheet 1/2: basics
	
	// PLEASE know the Polars user guide is amazing, open it now
	// https://pola-rs.github.io/polars-book/user-guide/introduction.html


	// Polars exposes the type DataFrame
	let df: DataFrame;

	// A DataFrame can be thought as a 2 dimentional excel spreadsheet:
	// It has multiple columns, each with multiple rows

	// You can read a file like so
	let df = CsvReader::from_path("data/countries.csv")?
		.has_header(true)
		// the `b` prefix is used to cast the `char` into a `u8`
		.with_delimiter(b',')
		.finish()?;


	// Or you can create one manually with the `df!` macro
	let df = df![
		"Country name" => ["Australia"      , "France"   , "United state", "Singapore", "Taiwan"   ],
		"Population"   => [25_690_000       , 67_390_000 , 330_000_000   , 5_686_000  , 23_570_000 ],
		"Area (sq km)" => [7_688_000_000_i64, 543_940_i64, 9_834_000_i64 , 728_i64    , 36_197_i64 ],
	]?;
	
	// We can print the dataframe beautifully with its debug format
	println!("Initial data");
	println!("{:?}", df);

	// We can use our dataframe like so:

	// We start with creating a 'lazy' execution framework. Unless special need, always use `.lazy()`:
	// this means it will wait for every query to be registered before executing them. This allows
	// the execution to only start after Polars have the full picture of the operations to do, and
	// allows for query optimisations.
	//
	// Note: the `.lazy()` requires the feature "lazy", see https://stackoverflow.com/questions/58480205/how-do-you-enable-a-rust-crate-feature
	let df = df.lazy()

		// We add the column 'Population density (pop/sq km)', the result of the formula Population / Area:
		.with_column((col("Population").strict_cast(DataType::Float32) / col("Area (sq km)")).alias("Population density (pop/sq km)"))
		// We select what columns we want
		.select([
			// We select the name
			col("Country name"),

			// We create a new column with a boolean to represent whether this country has highest population
			col("Population").eq(col("Population").max()).alias("Has highest population"),
			// Same with biggest area
			col("Area (sq km)").eq(col("Area (sq km)").max()).alias("Has biggest area"),

			// We select the population density
			col("Population density (pop/sq km)")
		])

		// and finally, we run the operations
		.collect()?;

	println!("Final data:");
	println!("{:?}", df);
	// prints
	// shape: (5, 4)
	// ┌──────────────┬────────────────────────┬──────────────────┬────────────────────────────────┐
	// │ Country name ┆ Has highest population ┆ Has biggest area ┆ Population density (pop/sq km) │
	// │ ---          ┆ ---                    ┆ ---              ┆ ---                            │
	// │ str          ┆ bool                   ┆ bool             ┆ f64                            │
	// ╞══════════════╪════════════════════════╪══════════════════╪════════════════════════════════╡
	// │ Australia    ┆ false                  ┆ true             ┆ 0.003342                       │
	// ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
	// │ France       ┆ false                  ┆ false            ┆ 123.892341                     │
	// ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
	// │ United state ┆ true                   ┆ false            ┆ 33.557047                      │
	// ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
	// │ Singapore    ┆ false                  ┆ false            ┆ 7810.4395                      │
	// ├╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
	// │ Taiwan       ┆ false                  ┆ false            ┆ 651.158936                     │
	// └──────────────┴────────────────────────┴──────────────────┴────────────────────────────────┘

	// And that's basically all Polar does. The better you get at writing the
	// formulas to express the content of a column, the better you get at Polars.
	// We call these 'formulas' Expressions.
	// Read more about them in the User guide (careful, the examples are written in python)
	// https://pola-rs.github.io/polars-book/user-guide/dsl/intro.html 

	return Ok(());
}
