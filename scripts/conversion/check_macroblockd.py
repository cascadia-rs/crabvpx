import glob, re

struct_regex = re.compile(r"#\[derive\([^)]*\)\]\n(?:#\[repr\(C\)\]\n)?pub struct Macroblockd\s*\{[\s\S]*?\}")
first = None
mismatches = []
for f in glob.glob("src/**/*.rs", recursive=True):
    with open(f, "r") as file:
        content = file.read()
    m = struct_regex.search(content)
    if m:
        if first is None:
            first = m.group(0)
            print(f"Captured from {f}")
        else:
            if m.group(0) != first:
                mismatches.append(f)
                
if not mismatches:
    print("All Macroblockd structs are identical!")
else:
    print("Mismatches found in:", len(mismatches), "files")
