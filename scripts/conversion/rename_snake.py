import os, glob, re

def load_renames(filename):
    renames = {}
    with open(filename, "r") as f:
        for line in f:
            if ":" in line:
                old, new = line.strip().split(":", 1)
                # Skip very short names to avoid accidents for now,
                # or handle them very carefully.
                if len(old) < 2 and old in ["C", "I", "J", "K", "L", "A", "B", "D", "E", "F", "G", "H", "M", "Q", "X"]:
                    continue
                renames[old] = new
    return renames

def apply_renames(renames):
    sorted_old_names = sorted(renames.keys(), key=len, reverse=True)
    if not sorted_old_names:
        print("No renames to apply.")
        return
        
    pattern = re.compile(r"\b(" + "|".join(re.escape(name) for name in sorted_old_names) + r")\b")

    count = 0
    for filepath in glob.glob("src/**/*.rs", recursive=True):
        if "scripts" in filepath: continue
        with open(filepath, "r") as f:
            content = f.read()
        
        orig = content
        
        def replace_match(m):
            name = m.group(1)
            start = m.start()
            
            # 1. Skip if preceded by "mod "
            lookback_mod = content[max(0, start-10):start]
            if re.search(r"\bmod\s+$", lookback_mod):
                return name
                
            # 2. Skip if inside #[repr(...)]
            # Check if there is a "#[repr(" before and a ")]" after
            # This is a bit heuristic but works for common cases
            prefix = content[max(0, start-20):start]
            if "#[repr(" in prefix:
                return name
                
            return renames[name]
            
        content = pattern.sub(replace_match, content)
        
        if orig != content:
            with open(filepath, "w") as f:
                f.write(content)
            count += 1
            
    print(f"Renamed snake_case violations in {count} files.")

if __name__ == "__main__":
    # We still need the renames file. I'll recreate it from the previous discovery.
    # Actually, I'll just use the list I saw earlier.
    renames_data = """BSwap64:bswap64
Border:border
DQ:dq
DQC:dqc
DQC_0:dqc_0
Delta:delta
FData:fdata
FLOATING_POINT_INIT:floating_point_init
FLOATING_POINT_RESTORE:floating_point_restore
Filter1:filter1
Filter2:filter2
GetCoeffs:get_coeffs
GetSigned:get_signed
HFilter:hfilter
Height:height
Left:left
Len:len
MBs:mbs
Pfactor:pfactor
QIndex:qindex
Round:round
Scale1Dh:scale1_dh
Scale1Dv:scale1_dv
Scale2D:scale2_d
Temp:temp
UPtr:uptr
UVdequant:uvdequant
VFilter:vfilter
VPtr:vptr
Version:version
Width:width
Y1dequant:y1dequant
Y2dequant:y2dequant
YPtr:yptr"""
    renames = {}
    for line in renames_data.split("\n"):
        if ":" in line:
            old, new = line.strip().split(":", 1)
            renames[old] = new
    apply_renames(renames)