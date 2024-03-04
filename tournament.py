from rps import play
import polars as pl
import glob
import re

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


for a in range(len(codes)):
    for b in range(a + 1, len(codes)):
        print(f"Playing {js_files[a]} against {js_files[b]}")
        score_a, score_b = play(codes[a], codes[b])
        scores[a] += score_a
        scores[b] += score_b

for i in range(len(codes)):
    scores[i] = float(scores[i]) / (len(codes) - 1)

result = pl.DataFrame(
    {"Name": names, "Author": authors, "Description": descriptions, "Score": scores}
)
result = result.sort("Score")
print(result)
