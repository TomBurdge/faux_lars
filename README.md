# Faux-lars
## Specification
* Polars plugin.
* Generates fake data.
* Generates random data with v fast vec macro.
* Generates rng data with 

## Behind the scenes
supported by rust equivalent function:
* faker.make_series -> type: str (consider polars native type too, handled by python), length: int.

pure python:
* faker.make_df -> args: schema: dict, length: int, rng: list = None
* faker.make_lf (ibid. lazy)

Very limited public API.
People can do arrays/structs with PRs themselves if there is popularity.

## How the code works
(this is the most flexible/liable to change with learning - e.g. since is dependent on whether has rng arg in first place, might not quite be `from_str` implementation)
* Implements an enum with `from_str` for each type. (Don't bother with language arg, just keep as strings).
* Implements two methods for each enum (trait??): `create`, and `create_with_rng`.
