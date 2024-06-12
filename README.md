# Faux-lars
`fauxlars` is a speedy fake data generation library.


## Example
Install the library
```sh
python -m venv .venv && source .venv/bin/activate && python -m pip install faux_lars
```
Let's generate some data
```python
import polars as pl
from faux_lars import generate_lazyframe

rows =5000
df = (
    generate_lazyframe(
        {
            "clicks": pl.UInt8,
            "Name": "name",
            "City Name": "city_name",
            "Phone": "mobile_number",
        },
        rows,
        "en",
    ).collect()
)
print(df)
```
```python
┌────────┬──────────────────┬──────────────────────────┬────────────────┐
│ clicks ┆ Name             ┆ City Name                ┆ Phone          │
│ ---    ┆ ---              ┆ ---                      ┆ ---            │
│ u8     ┆ str              ┆ str                      ┆ str            │
╞════════╪══════════════════╪══════════════════════════╪════════════════╡
│ 207    ┆ Erik Conn        ┆ Beier stad               ┆ 512-797-9451   │
│ 66     ┆ Hildegard Murphy ┆ Wintheiser bury          ┆ (907) 011-3125 │
│ 38     ┆ Tremayne Casper  ┆ Okuneva land             ┆ 933.955.5987   │
│ 157    ┆ Kamille Haley    ┆ McGlynn furt             ┆ 329.524.7080   │
│ 123    ┆ Erika Kozey      ┆ East Favian Fisher burgh ┆ 1-846-364-8772 │
│ …      ┆ …                ┆ …                        ┆ …              │
│ 25     ┆ Chadd Rosenbaum  ┆ Nader furt               ┆ 201-837-7966   │
│ 50     ┆ Tevin Jerde      ┆ Davis shire              ┆ 1-125-407-6570 │
│ 22     ┆ Antwan Jones     ┆ West Sammie Hirthe shire ┆ 365-256-8860   │
│ 120    ┆ Eden McCullough  ┆ Goldner shire            ┆ 1-950-582-2326 │
│ 76     ┆ Tevin Batz       ┆ Bauch mouth              ┆ 560-202-3844   │
└────────┴──────────────────┴──────────────────────────┴────────────────┘
```

## Docs
The `faux_lars` python API is deliberately small and lightweight.

There are two functions:
* `generate_lazyframe`
* `generate_dataframe`

Each receive the following arguments:
* `schema`: A dictionary of `column_name`, `data_type` key, value pairs. `data_type` can be a string `data_type` in the table below, or a non-complex polars DataType object.
* `rows`: The number of rows to generate.
* `language`: Any of the supported locales; see the table below.

## Supported Locales

| Data Type                    | en  | fr  | jp  | ar  | pt_br | zh_tw | zh_cn |
|------------------------------|-----|-----|-----|-----|-------|-------|-------|
| polars_datatypes             | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| name                         | ✓   | ✓   | ✓   | ✓   | ✓     | ✓     | ✓     |
| first_name                   | ✓   | ✓   | ✓   | ✓   | ✓     | ✓     | ✓     |
| last_name                    | ✓   | ✓   | ✓   | ✓   | ✓     | ✓     | ✓     |
| building_number              | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| city_name                    | ✓   |     |     |     |       |       |       |
| country_code                 | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| country_name                 | ✓   |     |     |     |       |       |       |
| latitude                     | ✓   |     |     |     |       |       |       |
| longitude                    | ✓   |     |     |     |       |       |       |
| postcode                     | ✓   |     |     |     |       |       |       |
| secondary_address            | ✓   |     |     |     |       |       |       |
| secondary_address_type       | ✓   |     |     |     |       |       |       |
| state_abbr                   | ✓   |     |     |     |       |       |       |
| state_name                   | ✓   |     |     |     | ✓     |       |       |
| street_name                  | ✓   |     |     |     | ✓     |       |       |
| time_zone                    | ✓   |     |     |     |       |       |       |
| zip_code                     | ✓   |     |     |     |       |       |       |
| isbn                         | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| isbn_10                      | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| isbn_13                      | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| bs                           | ✓   |     |     |     |       |       |       |
| company_name                 | ✓   | ✓   | ✓   | ✓   | ✓     | ✓     | ✓     |
| industry                     | ✓   |     |     |     |       |       |       |
| profession                   | ✓   |     |     |     |       |       |       |
| credit_card_number           | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| currency_code                | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| currency_name                | ✓   |     |     |     |       |       |       |
| currency_symbol              | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| dir_path                     | ✓   |     |     |     |       |       |       |
| file_ext                     | ✓   |     |     |     |       |       |       |
| file_name                    | ✓   |     |     |     |       |       |       |
| file_path                    | ✓   |     |     |     |       |       |       |
| mime_type                    | ✓   |     |     |     |       |       |       |
| semver                       | ✓   |     |     |     |       |       |       |
| semver_stable                | ✓   |     |     |     |       |       |       |
| semver_unstable              | ✓   |     |     |     |       |       |       |
| licence_plate                |     | ✓   |     |     |       |       |       |
| health_insurance_code        |     | ✓   |     |     |       |       |       |
| free_email                   | ✓   | ✓   |     |     | ✓     |       |       |
| bic                          | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| isin                         | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| ip                           | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| ipv4                         | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| ipv6                         | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| mac_address                  | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| password                     | ✓   | ✓   |     |     | ✓     |       |       |
| safe_email                   | ✓   | ✓   | ✓   | ✓   | ✓     | ✓     | ✓     |
| user_agent                   | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| user_name                    | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| field                        | ✓   |     | ✓   |     |       |       | ✓     |
| position                     | ✓   |     | ✓   |     |       |       | ✓     |
| seniority                    | ✓   |     | ✓   |     |       |       | ✓     |
| title                        | ✓   |     | ✓   |     |       |       | ✓     |
| lorem                        | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| cell_number                  | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |
| phone_number                 | ✓*  | ✓*  | ✓*  | ✓*  | ✓*    | ✓*    | ✓*    |

Key:
* ✓
* ✓ : supported for this locale.
* ✓* : supported, but this value does not vary by locale.

**All** non-complex polars DataType objects are supported. These can be passed by name as a string, **or** as a polars DataType object (see the example above).

For non-sring types, locale is irrelevant. A complex polars Datatype is a `struct`, `array`, `list`, or `enum`.

Locales:
* `en`: English
* `fr`: French
* `jp`: Japanese
* `ar`: Saudi Arabian Arabic
* `pt_br`: Brazilian Portugese
* `zh_tw`: Traditional Chinese
* `zh_cn`: Simplified Chinese


## Benchmarks
* On a laptop with 16GB of RAM and 8 cores, 5,000 rows with 1 utf8 column and three string columns generates in under 0.25 seconds.
* 1,000,000 rows generates in under 7. 
* Take a look at `benchmarks` for  comparison with two popular python fake data generation libraries: `mimesis` and `faker`.
