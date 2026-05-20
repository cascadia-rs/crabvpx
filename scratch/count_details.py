import os
import re

workspace = "/usr/local/google/home/liberato/src/crabvpx"
src_dir = os.path.join(workspace, "src")

out_of_scope_files = [
    "vp8/vp8_dx_iface.rs",
    "vpx/src/vpx_encoder.rs",
    "vpx/src/vpx_codec.rs",
    "vpx/src/vpx_decoder.rs",
    "vpx/src/vpx_image.rs",
    "api.rs"
]

total_unsafe = 0
out_of_scope_unsafe = 0
in_scope_unsafe = 0

unsafe_re = re.compile(r'\bunsafe\b')

for root, dirs, files in os.walk(src_dir):
    for file in files:
        if file.endswith(".rs"):
            full_path = os.path.join(root, file)
            rel_path = os.path.relpath(full_path, src_dir)
            
            try:
                with open(full_path, 'r', encoding='utf-8') as f:
                    content = f.read()
                    matches = len(unsafe_re.findall(content))
                    total_unsafe += matches
                    
                    if rel_path in out_of_scope_files:
                        out_of_scope_unsafe += matches
                        print(f"Out-of-scope: {rel_path} -> {matches}")
                    else:
                        in_scope_unsafe += matches
                        if matches > 0:
                            print(f"In-scope: {rel_path} -> {matches}")
            except Exception as e:
                print(f"Error reading {rel_path}: {e}")

print(f"\nTotal unsafe: {total_unsafe}")
print(f"Out-of-scope unsafe: {out_of_scope_unsafe}")
print(f"In-scope unsafe: {in_scope_unsafe}")
