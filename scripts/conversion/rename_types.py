import os, glob, re, json

def to_pascal_case(name):
    if name.lower().startswith("c2rustunnamed"): return name
    parts = re.split(r"_+", name)
    return "".join(p.capitalize() for p in parts if p)

def main():
    # 1. Discover TYPES only
    type_definitions = set()
    for filepath in glob.glob("src/**/*.rs", recursive=True):
        if "scripts" in filepath: continue
        with open(filepath, "r") as f: content = f.read()
        for match in re.finditer(r"\b(struct|enum|union|type)\s+([a-zA-Z0-9_]+)", content):
            name = match.group(2)
            if name in ["u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "usize", "isize", "f32", "f64", "bool", "SizeT", "PtrdiffT"]:
                continue
            if name.lower().startswith("c2rustunnamed"): continue
            if "_" in name or (name[0].islower()):
                type_definitions.add(name)

    keywords = {"as", "for", "if", "in", "let", "mod", "mut", "pub", "ref", "use", "where", "while", "type", "struct", "enum", "union", "trait", "impl", "fn", "unsafe", "extern", "static", "const"}
    renames = {name: to_pascal_case(name) for name in type_definitions if name not in keywords}

    sorted_old = sorted(renames.keys(), key=len, reverse=True)
    if not sorted_old: return
    
    pattern = re.compile(r"(?<!mod\s)\b(" + "|".join(re.escape(o) for o in sorted_old) + r")\b")

    for filepath in glob.glob("src/**/*.rs", recursive=True):
        if "scripts" in filepath: continue
        with open(filepath, "r") as f: content = f.read()
        orig = content
        
        # Cleanup circular aliases BEFORE replacement
        lines = content.split("\n")
        new_lines = []
        for line in lines:
            m = re.match(r"^\s*(pub\s+)?type\s+([a-zA-Z0-9_]+)\s*=\s*([a-zA-Z0-9_]+)\s*;", line)
            if m:
                alias, target = m.group(2), m.group(3)
                if renames.get(alias, alias) == renames.get(target, target):
                    if "api.rs" in filepath and alias == "Frame": pass
                    else: continue
            new_lines.append(line)
        content = "\n".join(new_lines)

        content = pattern.sub(lambda m: renames[m.group(1)], content)
        
        if orig != content:
            with open(filepath, "w") as f: f.write(content)

if __name__ == "__main__":
    main()