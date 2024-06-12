import polars as pl
from faux_lars import generate_dataframe
from time import time

rows = 1_000_000

ts = time()
df = (
    generate_dataframe(
        {
            "clicks": pl.UInt8,
            "Name": "name",
            "Email": "safe_email",
            "Phone": "mobile_number",
        },
        rows,
        "en",
    ),
)

te = time()
total_time = te - ts
print(f"Processing {rows} rows with faux_lars  took {total_time} seconds.)")
print(df)
