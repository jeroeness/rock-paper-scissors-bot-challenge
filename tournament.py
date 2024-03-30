from rps import play
import polars as pl
import glob
import re
from tqdm import tqdm
import random


class Agent:
    def __init__(self, name, author, code, description):
        self.name = name
        self.author = author
        self.code = code
        self.description = description
        self.score = 0


agents = []

js_files = glob.glob("./agents/*.js")
for file in js_files:
    with open(file, "r") as f:
        code = f.read()
        agents.append(
            Agent(
                re.search(r"// Name: (.+)", code).group(1),
                re.search(r"// Author: (.+)", code).group(1),
                code,
                re.search(r"// Description: (.+)", code).group(1),
            )
        )


matches = [[a, b] for a in range(len(agents)) for b in range(a, len(agents))]
total_rounds = 1000
print(f"Playing {total_rounds} rounds")
for a, b in tqdm(matches):
    score_a, score_b = play(agents[a].code, agents[b].code)
    agents[a].score += score_a
    agents[b].score += score_b

for i in range(len(agents)):
    agents[i].score = round(
        float(agents[i].score) / len(agents) / total_rounds * 1000, 2
    )


result = pl.DataFrame(
    {
        "Rank": list(range(1, len(agents) + 1)),
        "Name": [a.name for a in agents],
        "Author": [a.author for a in agents],
        "Description": [a.description for a in agents],
        "Score": [a.score for a in agents],
    }
)
result = result.sort("Score", descending=True)
result = result.with_columns(pl.Series(list(range(1, len(agents) + 1))).alias("Rank"))
result = result.head(20)


with open("README.md", "r") as f:
    readme = f.read()
    with pl.Config(
        tbl_formatting="ASCII_MARKDOWN",
        tbl_rows=10**9,
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
