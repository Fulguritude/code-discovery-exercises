use polars::prelude::*;

// We return a Result to be able to use the `?` operator. 
// Even if the failure case is not handled in main, at least this file is more readable
fn polars_advanced() -> Result<()>
{
	// Polars cheatsheet 2/2: advanced
	
	// Let's do an example with something we do a lot in data science: handling time-series
	// We track the number of sales of an articles (julst imagine there's records for every days)
	let df = df![
		// note: There are a few inconsistencies:         v here                  v here       v and here
		"Date"  => ["14/04/2022", "30/04/2022", "18/5/2022", "30/05/2022", "1/06/2022", "2/06/2022", "14/06/2022"],
		"Sales" => [           2,            5,           2,            0,           1,           3,            5],
	]?;
	const ITEM_PRICE:i32 = 10;

	println!("Initial data");
	println!("{:?}", df);

	// We want to know the number of sales per month
	
	// First we need to parse the date to a format rust understands
	// This time, we use the `with_column` to overwrite the "Date" from a string to a Date type
	let options =
		StrpTimeOptions
		{
			// The format, see https://docs.rs/chrono/latest/chrono/format/strftime/index.html
			// The '-' modifier means that it accepts days and months even without padding zero
			fmt: Some("%-d/%-m/%Y".into()),
			// This is important, it doesn't work without it, but I can't quite explain why :(
			exact: true,
			// We want a `Date`, not a `DateTime`
			date_dtype: DataType::Date,
			// Return an error if any date fails to get parsed
			strict: false,
		};

	let df =
		df
		.lazy()
		// Note: the `.str()` requires the feature "strings"
		.with_column(col("Date").str().strptime(options))
		.collect()?;


	let df_monthly =
		df
		.lazy()
		// We group the rows based on the result of formatting the date as "%Y-%m",
		// so we get one group per month
		.groupby([col("Date").dt().strftime("%Y-%m").alias("Month")]) 
		// Note: we could have more than one column acting as a group key, in which case there
		// will be one group per unique combinaison of these group keys columns

		// This 'groupby' returns a `LazyGroupBy`, instead of a `LazyFrame`
		// We are now in a 'groupby' context (instead of a simple 'select' context).
		// This means we have access to extra functionalities to handle our groups
		// We define a new set of column and how we want to aggregate the rows of each group into a
		// single row.
		// The first column of the new DataFrame is the 'groupby' key column
		.agg
		(
			[
				 col("Sales").sum().alias("Monthly sales count"),
				(col("Sales").sum() * lit(ITEM_PRICE)).alias("Monthly revenue"),
			]
		)

		// We now have all the data grouped by month
		// We add another new column holding the monthly revenue
		.with_column
		(
			col("Monthly revenue")
			// We will be mapping the number of sales to a custom message describing how good the
			// month is. The mapping function we want to write is of type int -> &str.
			// However, unlike in python, the `map` method of an expression takes and returns an
			// entire series.
			// This is ok because series are all lazy-evaluated. This allows polar to execute our
			// callback not to *do the mapping*, but to *explain how the mapping will be done*.
			// This allows polars to get an overview of what work needs to be done and optimize the
			// queries before starting the computations.
			//
			// see https://pola-rs.github.io/polars-book/user-guide/dsl/custom_functions.html
			.map
			(
				|series: Series|
				{
					// Series are type-agnostic.
					// If we want to use the values within it, we first need to read the Series 
					// as a certain type.
					let new_series =
						series
						.i32()?
						.into_iter()

						// We can finally describe how each of these values will be mapped
						.map
						(
							|revenue| -> &str
							{
								// Series support null values, so values are given as std::option
								if let Some(revenue) = revenue
								{
									if revenue < 0
									{
										return "We're a literal charity";
									}
									else if revenue < 50
									{
										return "Bad :(";
									}
									else if revenue < 100
									{
										return "Good :)";
									}
									else
									{
										return "Excellent ! :D";
									}
								}
								else
								{
									return "Why is there no revenue data for this month ?!";
								}
							}
						)
						.collect();
					return Ok(new_series);
				},
				GetOutput::from_type(DataType::Utf8)
			)
			.alias("Monthly mood")
		)
		.collect()?;

	println!("Final data");
	println!("{:?}", df_monthly);
	// prints
	//
	// shape: (3, 4)
	// ┌─────────┬─────────────────────┬─────────────────┬──────────────┐
	// │ Month   ┆ Monthly sales count ┆ Monthly revenue ┆ Monthly mood │
	// │ ---     ┆ ---                 ┆ ---             ┆ ---          │
	// │ str     ┆ i32                 ┆ i32             ┆ str          │
	// ╞═════════╪═════════════════════╪═════════════════╪══════════════╡
	// │ 2022-05 ┆ 2                   ┆ 20              ┆ Bad :(       │
	// ├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
	// │ 2022-04 ┆ 7                   ┆ 70              ┆ Good :)      │
	// ├╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌╌┼╌╌╌╌╌╌╌╌╌╌╌╌╌╌┤
	// │ 2022-06 ┆ 9                   ┆ 90              ┆ Good :)      │
	// └─────────┴─────────────────────┴─────────────────┴──────────────┘

	return Ok(());
}
