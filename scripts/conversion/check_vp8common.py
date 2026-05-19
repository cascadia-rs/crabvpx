import glob, re

count = 0
for f in glob.glob("src/**/*.rs", recursive=True):
    with open(f, "r") as file:
        content = file.read()
    if re.search(r"pub struct VP8Common\b", content):
        count += 1
print(f"VP8Common: {count}")
