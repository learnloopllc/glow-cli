#!/usr/bin/env python3
"""Generate Rust types from an OpenAPI spec.

Usage:
    python3 generate-rust-types.py <openapi.json> [source_name] [source_version]

Reads the components/schemas section and emits Rust structs with
serde Serialize/Deserialize derives. Output goes to stdout.
"""

import json
import re
import sys

# Rust reserved keywords that need raw identifier syntax
RUST_KEYWORDS = {
    "as", "break", "const", "continue", "crate", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod",
    "move", "mut", "pub", "ref", "return", "self", "Self", "static", "struct",
    "super", "trait", "true", "type", "unsafe", "use", "where", "while",
    "async", "await", "dyn", "abstract", "become", "box", "do", "final",
    "macro", "override", "priv", "typeof", "unsized", "virtual", "yield",
    "try",
}


def sanitize_name(name: str) -> str:
    """Convert OpenAPI schema name to valid Rust type name."""
    # Replace hyphens (ActivityResponse-Input → ActivityResponseInput)
    return name.replace("-", "")


def safe_field(name: str) -> tuple[str, str | None]:
    """Return (rust_field_name, serde_rename_or_none)."""
    if name in RUST_KEYWORDS:
        return f"r#{name}", f'#[serde(rename = "{name}")]'
    return name, None


def resolve_type(prop: dict, required: bool = True) -> str:
    """Convert an OpenAPI property schema to a Rust type string."""
    # $ref
    if "$ref" in prop:
        ref_name = prop["$ref"].split("/")[-1]
        rust_type = sanitize_name(ref_name)
        return rust_type if required else f"Option<{rust_type}>"

    # anyOf — nullable pattern
    if "anyOf" in prop:
        types = prop["anyOf"]
        non_null = [t for t in types if t.get("type") != "null"]
        has_null = any(t.get("type") == "null" for t in types)
        if len(non_null) == 1:
            inner = resolve_type(non_null[0], required=True)
            return f"Option<{inner}>" if has_null else inner
        return "Option<serde_json::Value>" if has_null else "serde_json::Value"

    # allOf — usually single ref with description
    if "allOf" in prop:
        if len(prop["allOf"]) == 1:
            return resolve_type(prop["allOf"][0], required)
        return "serde_json::Value"

    # oneOf — too complex, use Value
    if "oneOf" in prop:
        return "serde_json::Value"

    prop_type = prop.get("type")

    if prop_type == "string":
        rust_type = "String"
    elif prop_type == "integer":
        rust_type = "i64"
    elif prop_type == "number":
        rust_type = "f64"
    elif prop_type == "boolean":
        rust_type = "bool"
    elif prop_type == "array":
        items = prop.get("items", {})
        item_type = resolve_type(items, required=True)
        rust_type = f"Vec<{item_type}>"
    elif prop_type == "object":
        additional = prop.get("additionalProperties")
        if additional:
            if additional is True:
                rust_type = "serde_json::Value"
            elif isinstance(additional, dict):
                value_type = resolve_type(additional, required=True)
                rust_type = f"std::collections::HashMap<String, {value_type}>"
            else:
                rust_type = "serde_json::Value"
        else:
            rust_type = "serde_json::Value"
    elif prop_type is None:
        # No type specified — could be a composed schema
        rust_type = "serde_json::Value"
    else:
        rust_type = "serde_json::Value"

    if not required and not rust_type.startswith("Option<"):
        rust_type = f"Option<{rust_type}>"
    return rust_type


def generate_enum(name: str, schema: dict) -> str:
    """Generate a Rust enum from an OpenAPI enum schema."""
    rust_name = sanitize_name(name)
    variants = schema["enum"]
    lines = [
        "#[derive(Debug, Clone, Serialize, Deserialize)]",
        f"pub enum {rust_name} {{",
    ]
    for v in variants:
        if not isinstance(v, str):
            continue
        # Build a PascalCase variant name
        cleaned = re.sub(r"[^a-zA-Z0-9_]", "_", v)
        parts = cleaned.split("_")
        variant = "".join(p.capitalize() for p in parts if p)
        if not variant:
            variant = f"Value{v}"
        # Ensure it starts with a letter
        if variant[0].isdigit():
            variant = f"V{variant}"
        lines.append(f'    #[serde(rename = "{v}")]')
        lines.append(f"    {variant},")
    lines.append("}")
    return "\n".join(lines)


def generate_struct(name: str, schema: dict) -> str:
    """Generate a Rust struct from an OpenAPI schema."""
    rust_name = sanitize_name(name)
    properties = schema.get("properties", {})
    required_fields = set(schema.get("required", []))

    if not properties:
        return f"pub type {rust_name} = serde_json::Value;"

    lines = [
        "#[derive(Debug, Clone, Serialize, Deserialize)]",
        f"pub struct {rust_name} {{",
    ]

    for field_name, field_schema in properties.items():
        is_required = field_name in required_fields
        rust_type = resolve_type(field_schema, required=is_required)

        # If not required and not already Option, wrap
        if not is_required and not rust_type.startswith("Option<"):
            rust_type = f"Option<{rust_type}>"

        rust_field, rename = safe_field(field_name)

        # Add serde attributes
        if rename:
            lines.append(f"    {rename}")
        if rust_type.startswith("Option<"):
            lines.append("    #[serde(default)]")

        lines.append(f"    pub {rust_field}: {rust_type},")

    lines.append("}")
    return "\n".join(lines)


def main():
    if len(sys.argv) < 2:
        print(
            "Usage: generate-rust-types.py <openapi.json> [source] [version]",
            file=sys.stderr,
        )
        sys.exit(1)

    with open(sys.argv[1]) as f:
        spec = json.load(f)

    source = sys.argv[2] if len(sys.argv) > 2 else spec.get("info", {}).get("title", "unknown")
    version = sys.argv[3] if len(sys.argv) > 3 else spec.get("info", {}).get("version", "unknown")
    schemas = spec.get("components", {}).get("schemas", {})

    print(f"// Auto-generated from {source} OpenAPI spec v{version}")
    print("// Do not edit manually — regenerated on each API release.")
    print(f"// Schemas: {len(schemas)}")
    print()
    print("#![allow(dead_code, clippy::derive_partial_eq_without_eq)]")
    print()
    print("use serde::{Deserialize, Serialize};")
    print()

    for name in sorted(schemas.keys()):
        schema = schemas[name]

        # Enum
        if "enum" in schema:
            print(generate_enum(name, schema))
            print()
            continue

        # Struct (has properties or is declared as object)
        if schema.get("type") == "object" or "properties" in schema:
            print(generate_struct(name, schema))
            print()
            continue

        # Simple type alias (string, integer, etc.) — skip, rarely useful
        # Composed schemas without properties — type alias to Value
        if "allOf" in schema or "oneOf" in schema or "anyOf" in schema:
            rust_name = sanitize_name(name)
            print(f"pub type {rust_name} = serde_json::Value;")
            print()


if __name__ == "__main__":
    main()
