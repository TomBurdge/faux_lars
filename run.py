import polars as pl
from faux_lars import generate_dataframe
from time import time
from mimesis import Fieldset
from mimesis.locales import Locale
rows = 1_000_000

fs = Fieldset(locale=Locale.EN, i=rows)
ts = time()
df = pl.DataFrame({
    "Name": fs("person.full_name"),
    "Email": fs("email"),
    "Phone": fs("telephone", mask="+1 (###) #5#-7#9#"),
})
te = time()
total_time = te - ts
print(f"Processing {rows} rows with faux_lars took {total_time} seconds.)")
print(df)

ts = time()
df = generate_dataframe({"Name": "name", "Email":"safe_email","Phone":"mobile_number"}, rows, "en"),

te = time()
total_time = te - ts
print(f"Processing {rows} rows with faux_lars  took {total_time} seconds.)")
print(df)

# print(df.unique())
