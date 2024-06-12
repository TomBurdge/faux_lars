#![allow(clippy::unused_unit)]
use crate::data_type::{DataType, FromKwargs};
use polars::prelude::*;
use pyo3_polars::derive::polars_expr;
use serde::Deserialize;
use std::string::String;

#[derive(Deserialize)]
pub struct Kwargs {
    pub length: usize,
    pub data_type: String,
    pub lang: String,
}

fn same_output_type(input_fields: &[Field]) -> PolarsResult<Field> {
    let field = &input_fields[0];
    Ok(field.clone())
}

macro_rules! create_data_type_series {
    ($type:ty, $length:expr) => {{
        let out = fake::vec![$type; $length];
        Series::new("out", out)
    }};
}

// TODO: chrono for date. Will need its own type handling and thought
// same for color fake::faker::color
#[polars_expr(output_type_func=same_output_type)]
fn generate_data(_inputs: &[Series], kwargs: Kwargs) -> PolarsResult<Series> {
    let length = kwargs.length;
    let out = match kwargs.data_type.as_str() {
        "binary" => create_data_type_series!(Vec<u8>, length),
        "bool" => create_data_type_series!(bool, length),
        "str" => create_data_type_series!(String, length),
        "i8" => create_data_type_series!(i8, length),
        "i16" => create_data_type_series!(i16, length),
        "i32" => create_data_type_series!(i32, length),
        "i64" => create_data_type_series!(i64, length),
        "u8" => create_data_type_series!(u8, length),
        "u16" => create_data_type_series!(u16, length),
        "u32" => create_data_type_series!(u32, length),
        "u64" => create_data_type_series!(u64, length),
        "f32" => create_data_type_series!(f32, length),
        "f64" => create_data_type_series!(f64, length),
        _ => {
            let data_type = DataType::from_kwargs(kwargs)?;
            data_type.generate()
        }
    };
    Ok(out)
}
