from __future__ import annotations

from pathlib import Path

import polars as pl

from faux_lars.utils import parse_into_expr, register_plugin, parse_version

if parse_version(pl.__version__) < parse_version("0.20.16"):
    from polars.utils.udfs import _get_shared_lib_location

    lib: str | Path = _get_shared_lib_location(__file__)
else:
    lib = Path(__file__).parent


def generate_data(
    expr: str, length: int, data_type: str, language: str = "en"
) -> pl.Expr:
    expression = parse_into_expr(expr)
    return register_plugin(
        args=[expression],
        symbol="generate_data",
        is_elementwise=True,
        changes_length=True,
        lib=lib,
        kwargs={"length": length, "data_type": data_type, "lang": language},
    )


data_types_to_str = {
    pl.Int8: "i8",
    pl.Int16: "i16",
    pl.Int32: "i32",
    pl.Int64: "i64",
    pl.UInt8: "u8",
    pl.UInt16: "u16",
    pl.UInt32: "u32",
    pl.UInt64: "u64",
    pl.Float32: "f32",
    pl.Float64: "f64",
    pl.Binary: "binary",
}


def generate_lazyframe(schema: dict, length: int, language: str = "en") -> pl.LazyFrame:
    data: dict = {name: [] for (name, _) in schema.items()}
    lf = pl.LazyFrame(data=data)
    lf = lf.select(
        *[
            generate_data(
                expr=name, length=length, data_type=col_type, language=language
            )
            if isinstance(col_type, str)
            else generate_data(
                expr=name, length=length, data_type=data_types_to_str[col_type]
            )
            for (name, col_type) in schema.items()
        ]
    )
    return lf


def generate_dataframe(schema: dict, length: int, language: str = "en") -> pl.DataFrame:
    lazyframe = generate_lazyframe(schema=schema, length=length, language=language)
    return lazyframe.collect()
