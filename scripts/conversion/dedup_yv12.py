import glob, re, os

struct_regex = re.compile(r"#\[derive\([^)]*\)\]\n#\[repr\(C\)\]\npub struct Yv12BufferConfig\s*\{[\s\S]*?\}")
source_file = "src/vpx_scale/generic/yv12config.rs"

for filepath in glob.glob("src/**/*.rs", recursive=True):
    if "scripts" in filepath: continue
    if os.path.abspath(filepath) == os.path.abspath(source_file):
        continue
        
    with open(filepath, "r") as file:
        content = file.read()
        
    m = struct_regex.search(content)
    if m:
        # Remove the struct
        content = content[:m.start()] + content[m.end():]
        
        # Add the import
        if "use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;" not in content:
            # Find a good place to insert (after standard uses or just at top)
            lines = content.split("\n")
            insert_idx = 0
            for i, line in enumerate(lines):
                if not line.startswith("#!"):
                    insert_idx = i
                    break
            lines.insert(insert_idx, "use crate::vpx_scale::generic::yv12config::Yv12BufferConfig;")
            content = "\n".join(lines)
            
        with open(filepath, "w") as file:
            file.write(content)
            
print("De-duplicated Yv12BufferConfig")