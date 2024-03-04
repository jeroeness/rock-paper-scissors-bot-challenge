from rps import play
import polars as pl
import glob
import re
from tqdm import tqdm

js_files = glob.glob("./agents/*.js")
codes = []
scores = []
names = []
authors = []
descriptions = []
for file in js_files:
    with open(file, "r") as f:
        code = f.read()
        codes.append(code)
        scores.append(0)
        authors.append(re.search(r"// Author: (.+)", code).group(1))
        names.append(re.search(r"// Name: (.+)", code).group(1))
        descriptions.append(re.search(r"// Description: (.+)", code).group(1)[0:50])

matches = [[a, b] for a in range(len(codes)) for b in range(a + 1, len(codes))]
for a, b in tqdm(matches):
    score_a, score_b = play(codes[a], codes[b])
    scores[a] += score_a
    scores[b] += score_b

for i in range(len(codes)):
    scores[i] = round(float(scores[i]) / (len(codes) - 1), 2)


result = pl.DataFrame(
    {
        "Rank": list(range(1, len(codes) + 1)),
        "Name": names,
        "Author": authors,
        "Description": descriptions,
        "Score": scores,
    }
)
result = result.sort("Score", descending=True)
result = result.with_columns(pl.Series(list(range(1, len(codes) + 1))).alias("Rank"))
result = result.head(20)


with open("README.md", "r") as f:
    readme = f.read()
    with pl.Config(
        tbl_formatting="ASCII_MARKDOWN",
        tbl_hide_column_data_types=True,
        tbl_hide_dataframe_shape=True,
    ):
        print(result)
        readme = re.sub(
            r"<!-- START TABLE -->.*<!-- END TABLE -->",
            f"<!-- START TABLE -->\n{result}\n<!-- END TABLE -->",
            readme,
            flags=re.DOTALL,
        )
with open("README.md", "w") as f:
    f.write(readme)
