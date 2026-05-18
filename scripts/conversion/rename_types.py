import os, glob, re

def to_pascal_case(name):
    if name.startswith("C2RustUnnamed"):
        return name.replace("C2RustUnnamed", "C2rustUnnamed")
    parts = re.split(r"_+", name)
    parts = [p for p in parts if p]
    return "".join(p.capitalize() for p in parts)

def main():
    # 1. Discover all renames
    type_definitions = set()
    for filepath in glob.glob("src/**/*.rs", recursive=True):
        if "scripts" in filepath: continue
        with open(filepath, "r") as f:
            content = f.read()
        for match in re.finditer(r"pub\s+(struct|enum|union|type)\s+([a-zA-Z0-9_]+)", content):
            name = match.group(2)
            if name and not (name[0].isupper() and "_" not in name):
                if name in ["size_t", "intptr_t", "uintptr_t", "ptrdiff_t", "ssize_t"]:
                    continue
                type_definitions.add(name)

    renames = {name: to_pascal_case(name) for name in type_definitions}
    sorted_old_names = sorted(renames.keys(), key=len, reverse=True)
    
    # 2. Apply renames
    # Stricter pattern: word boundary, but NOT preceded by "mod "
    # We use a negative lookbehind for "mod "
    pattern = re.compile(r"(?<!mod\s)\b(" + "|".join(re.escape(name) for name in sorted_old_names) + r")\b")

    count = 0
    for filepath in glob.glob("src/**/*.rs", recursive=True):
        with open(filepath, "r") as f:
            content = f.read()
        
        orig = content
        
        content = pattern.sub(lambda m: renames[m.group(1)], content)
        
        # 3. Remove circular type aliases: pub type NewName = NewName;
        # ONLY if they were actually created by our renaming
        # i.e. it was `pub type OLD = TARGET;` where `renames[OLD] == renames[TARGET]`
        # We can just look for the literal after-rename circularity.
        # EXCEPT we must not touch api.rs which has intentional `type Frame = Frame;`
        if "api.rs" not in filepath:
            content = re.sub(r"pub\s+type\s+([a-zA-Z0-9_]+)\s*=\s*\1\s*;", "", content)
            content = re.sub(r"type\s+([a-zA-Z0-9_]+)\s*=\s*\1\s*;", "", content)

        if orig != content:
            with open(filepath, "w") as f:
                f.write(content)
            count += 1
            
    print(f"Renamed types in {count} files.")

if __name__ == "__main__":
    main()