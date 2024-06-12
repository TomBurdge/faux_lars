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
            "Email": "safe_email",
            "Phone": "mobile_number",
        },
        rows,
        "en",
    ).collect()
)
print(df)
```
```python
shape: (5_000, 4)
┌────────┬──────────────────┬───────────────────────┬────────────────┐
│ clicks ┆ Name             ┆ Email                 ┆ Phone          │
│ ---    ┆ ---              ┆ ---                   ┆ ---            │
│ u8     ┆ str              ┆ str                   ┆ str            │
╞════════╪══════════════════╪═══════════════════════╪════════════════╡
│ 30     ┆ Pete Considine   ┆ hilton@example.org    ┆ 637-852-1570   │
│ 250    ┆ Berta Jones      ┆ billie@example.com    ┆ (103) 305-9278 │
│ 241    ┆ Bennett Nikolaus ┆ deontae@example.com   ┆ 113-777-7758   │
│ 206    ┆ Raymundo Roob    ┆ lavern@example.com    ┆ (786) 626-8033 │
│ 227    ┆ Alysha Larkin    ┆ margarita@example.net ┆ 172-282-6779   │
│ …      ┆ …                ┆ …                     ┆ …              │
│ 180    ┆ Eduardo Zboncak  ┆ sadye@example.com     ┆ 943-778-2835   │
│ 236    ┆ Camylle Conroy   ┆ connor@example.org    ┆ 131-634-2874   │
│ 51     ┆ Simone Gulgowski ┆ sydni@example.org     ┆ 1-346-844-6742 │
│ 30     ┆ Letitia Berge    ┆ jeffrey@example.net   ┆ 1-802-687-1114 │
│ 161    ┆ Anastacio Marks  ┆ eleanore@example.com  ┆ 1-276-014-3580 │
└────────┴──────────────────┴───────────────────────┴────────────────┘
```
On a laptop with 16GB of RAM and 8 cores, this generates in under half a second.

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
