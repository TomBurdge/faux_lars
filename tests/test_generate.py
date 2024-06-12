import polars as pl
from faux_lars import generate_lazyframe, generate_dataframe, data_types_to_str
import pytest
from typing import List

supported_columns_by_language = [
    (
        [
            "name",
            "first_name",
            "last_name",
            "building_number",
            "city_name",
            "country_name",
            "latitude",
            "longitude",
            "postcode",
            "secondary_address",
            "secondary_address_type",
            "state_abbr",
            "state_name",
            "street_name",
            "time_zone",
            "zip_code",
            "isbn",
            "isbn_10",
            "isbn_13",
            "bs",
            "company_name",
            "industry",
            "profession",
            "credit_card_number",
            "currency_code",
            "currency_name",
            "currency_symbol",
            "dir_path",
            "file_ext",
            "file_name",
            "file_path",
            "mime_type",
            "semver",
            "semver_stable",
            "semver_unstable",
            "free_email",
            "bic",
            "isin",
            "ip",
            "ipv4",
            "ipv6",
            "mac_address",
            "password",
            "safe_email",
            "user_agent",
            "user_name",
            "field",
            "position",
            "seniority",
            "title",
            "lorem",
            "cell_number",
            "mobile_number",
            "phone_number",
        ],
        "en",
        pl.Utf8,
    ),
    (
        [
            "name",
            "first_name",
            "last_name",
            "licence_plate",
            "health_insurance_code",
            "company_name",
            "free_email",
            "password",
            "safe_email",
        ],
        "fr",
        pl.Utf8,
    ),
    (
        [
            "name",
            "first_name",
            "last_name",
            "state_name",
            "street_name",
            "company_name",
            "free_email",
            "password",
            "safe_email",
        ],
        "pt_br",
        pl.Utf8,
    ),
    (
        [
            "name",
            "first_name",
            "last_name",
            "company_name",
            "safe_email",
            "field",
            "position",
            "title",
        ],
        "jp",
        pl.Utf8,
    ),
    (
        ["name", "first_name", "last_name", "company_name", "safe_email"],
        "ar",
        pl.Utf8,
    ),
    (
        ["name", "first_name", "last_name", "company_name", "safe_email"],
        "zh_tw",
        pl.Utf8,
    ),
    (
        [
            "name",
            "first_name",
            "last_name",
            "company_name",
            "safe_email",
            "field",
            "position",
            "seniority",
            "title",
        ],
        "zh_cn",
        pl.Utf8,
    ),
    (
        [
            "str",
        ],
        "en",
        pl.Utf8,
    ),
    (
        [
            "bool",
        ],
        "en",
        pl.Boolean,
    ),
]

data_type_columns = [
    (["binary", pl.Binary], "can be anything", pl.Binary),
    (["i8", pl.Int8], "can be anything", pl.Int8),
    (["i16", pl.Int16], "can be anything", pl.Int16),
    (["i32", pl.Int32], "can be anything", pl.Int32),
    (["i64", pl.Int64], "can be anything", pl.Int64),
    (["u8", pl.UInt8], "can be anything", pl.UInt8),
    (["u16", pl.UInt16], "can be anything", pl.UInt16),
    (["u32", pl.UInt32], "can be anything", pl.UInt32),
    (["u64", pl.UInt64], "can be anything", pl.UInt64),
    (["f32", pl.Float32], "can be anything", pl.Float32),
    (["f64", pl.Float64], "can be anything", pl.Float64),
]

supported_columns = data_type_columns + supported_columns_by_language


@pytest.mark.parametrize("""columns, lang, expected_d_type""", supported_columns)
def test_generate_lazyframe(columns: List, lang: str, expected_d_type: str):
    schema = {
        (column if isinstance(column, str) else data_types_to_str[column]): column
        for column in columns
    }
    result = generate_lazyframe(schema=schema, length=50, language=lang).collect()
    assert dict(result.schema) == {
        (
            column if isinstance(column, str) else data_types_to_str[column]
        ): expected_d_type
        for column in columns
    }


@pytest.mark.parametrize("""columns, lang, expected_d_type""", supported_columns)
def test_generate_dataframe(columns: List, lang: str, expected_d_type: str):
    schema = {
        (column if isinstance(column, str) else data_types_to_str[column]): column
        for column in columns
    }
    result = generate_dataframe(schema=schema, length=50, language=lang)
    assert dict(result.schema) == {
        (
            column if isinstance(column, str) else data_types_to_str[column]
        ): expected_d_type
        for column in columns
    }
