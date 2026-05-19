import glob, re

structs_to_check = [
    "Yv12BufferConfig",
    "Vp8Common",
    "Macroblockd",
    "Vp8dComp",
    "VpxCodecCtxT",
    "VpxImage",
]

for s in structs_to_check:
    count = 0
    for f in glob.glob("src/**/*.rs", recursive=True):
        with open(f, "r") as file:
            content = file.read()
        if re.search(r"pub struct " + s + r"\b", content):
            count += 1
    print(f"{s}: {count}")
