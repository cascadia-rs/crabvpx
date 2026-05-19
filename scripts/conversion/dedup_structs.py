import glob, re, os

structs = [
    {
        "name": "Macroblockd",
        "source": "src/vp8/decoder/onyxd_if.rs",
        "import_path": "use crate::vp8::decoder::onyxd_if::Macroblockd;"
    },
    {
        "name": "Vp8dComp",
        "source": "src/vp8/decoder/onyxd_if.rs",
        "import_path": "use crate::vp8::decoder::onyxd_if::Vp8dComp;"
    },
    {
        "name": "VpxImage",
        "source": "src/vpx/src/vpx_image.rs",
        "import_path": "use crate::vpx::src::vpx_image::VpxImage;"
    }
]

for s in structs:
    struct_regex = re.compile(r"#\[derive\([^)]*\)\]\n(?:#\[repr\(C\)\]\n)?pub struct " + s["name"] + r"\s*\{[\s\S]*?\}")
    for filepath in glob.glob("src/**/*.rs", recursive=True):
        if "scripts" in filepath: continue
        if os.path.abspath(filepath) == os.path.abspath(s["source"]):
            continue
            
        with open(filepath, "r") as file:
            content = file.read()
            
        m = struct_regex.search(content)
        if m:
            content = content[:m.start()] + content[m.end():]
            
            if s["import_path"] not in content:
                lines = content.split("\n")
                insert_idx = 0
                for i, line in enumerate(lines):
                    if not line.startswith("#!"):
                        insert_idx = i
                        break
                lines.insert(insert_idx, s["import_path"])
                content = "\n".join(lines)
                
            with open(filepath, "w") as file:
                file.write(content)
                
    print(f"De-duplicated {s['name']}")