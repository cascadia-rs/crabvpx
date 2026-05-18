import os, glob, re, json

def to_pascal_case(name):
    if name.lower().startswith("c2rustunnamed"):
        return name
    parts = re.split(r"_+", name)
    return "".join(p.capitalize() for p in parts if p)

def main():
    primitives = {"u8", "u16", "u32", "u64", "i8", "i16", "i32", "i64", "usize", "isize", "f32", "f64", "bool", "str", "char"}
    
    # 1. Discover all type aliases
    all_aliases = {}
    for filepath in glob.glob("src/**/*.rs", recursive=True):
        if "scripts" in filepath: continue
        with open(filepath, "r") as f:
            content = f.read()
        for match in re.finditer(r"(?:pub\s+)?type\s+([a-zA-Z0-9_]+)\s*=\s*([a-zA-Z0-9_]+)\s*;", content):
            alias, target = match.groups()
            all_aliases[alias] = target

    # 2. Resolve aliases to primitives
    def resolve(name, visited):
        if name in primitives: return name
        if name in visited: return None
        if name in all_aliases:
            return resolve(all_aliases[name], visited | {name})
        return None

    resolved_primitives = {}
    for alias in all_aliases:
        if alias.lower().startswith("c2rustunnamed"): continue
        res = resolve(alias, set())
        if res:
            resolved_primitives[alias] = res
            
    # 3. Discover PascalCase renames
    pascal_renames = {}
    for filepath in glob.glob("src/**/*.rs", recursive=True):
        if "scripts" in filepath: continue
        with open(filepath, "r") as f:
            content = f.read()
        for match in re.finditer(r"\b(struct|enum|union|type)\s+([a-zA-Z0-9_]+)", content):
            name = match.group(2)
            if name in primitives or name in resolved_primitives or name.lower().startswith("c2rustunnamed"):
                continue
            if "_" in name or (name[0].islower() and name not in primitives):
                pascal_renames[name] = to_pascal_case(name)

    # Keywords
    keywords = {
        "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for", 
        "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", 
        "self", "Self", "static", "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while",
        "async", "await", "dyn", "abstract", "become", "box", "do", "final", "macro", "override", "priv", 
        "typeof", "unsized", "virtual", "yield", "try"
    }

    all_replacements = pascal_renames.copy()
    all_replacements.update(resolved_primitives)
    
    user_requested = {
        "Int8T": "i8", "Int16T": "i16", "Int32T": "i32", "Int64T": "i64",
        "Uint8T": "u8", "Uint16T": "u16", "Uint32T": "u32", "Uint64T": "u64",
        "int8_t": "i8", "int16_t": "i16", "int32_t": "i32", "int64_t": "i64",
        "uint8_t": "u8", "uint16_t": "u16", "uint32_t": "u32", "uint64_t": "u64",
    }
    all_replacements.update(user_requested)
    all_replacements = {k: v for k, v in all_replacements.items() if k and k not in keywords and v not in keywords}

    sorted_old_names = sorted(all_replacements.keys(), key=len, reverse=True)
    pattern = re.compile(r"(?<!mod\s)\b(" + "|".join(re.escape(name) for name in sorted_old_names) + r")\b")

    for filepath in glob.glob("src/**/*.rs", recursive=True):
        if "scripts" in filepath: continue
        with open(filepath, "r") as f:
            content = f.read()
        
        orig = content
        
        # 4. Strip redundant type aliases
        lines = content.split("\n")
        new_lines = []
        for line in lines:
            m = re.match(r"^\s*(pub\s+)?type\s+([a-zA-Z0-9_]+)\s*=\s*([a-zA-Z0-9_]+)\s*;", line)
            if m:
                alias, target = m.group(2), m.group(3)
                # Drop if we have a replacement AND (it is circular OR target is a primitive)
                if alias in all_replacements:
                    repl = all_replacements[alias]
                    if repl == alias or repl in primitives:
                        if "api.rs" in filepath and alias == "Frame":
                            pass
                        elif alias in ["SizeT", "PtrdiffT"]:
                            pass
                        else:
                            continue
                # Also drop if it's already circular after renaming both sides
                if all_replacements.get(alias, alias) == all_replacements.get(target, target):
                    if "api.rs" in filepath and alias == "Frame":
                        pass
                    elif all_replacements.get(alias, alias) in ["SizeT", "PtrdiffT"]:
                        pass
                    else:
                        continue
            new_lines.append(line)
        content = "\n".join(new_lines)
        
        # 5. Apply replacements
        content = pattern.sub(lambda m: all_replacements[m.group(1)], content)
        
        if orig != content:
            with open(filepath, "w") as f: f.write(content)

if __name__ == "__main__":
    main()