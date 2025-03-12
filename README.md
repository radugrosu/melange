# Melange

## Overview

Melange is an AI-powered code linter automating enforcement of rules specified in natural language by users.

The rules exist at three levels:

- project scope: .melangerules file
- file scope: top of the file
- block scope: decorating struct/function/module

## Examples

Rules defined in the .melangerules file:

```bash
> cat .melangerules
AIRULE: For Python code, conform to PEP8
AIRULE: For Rust Code, don't enforce safety checks
AIRULE: For every hook in src/layer/* make sure a test is implemented in e2e/file.rs

```

Top level rules:

```bash
> cat example.py
# AIRULE: Arguments on first line forbidden when not using vertical alignment

foo = long_function_name(var_one, var_two,
    var_three, var_four)
```

Running `melage example.py` will output:

```bash
foo = long_function_name(var_one, var_two,
                         ^        ^
    var_three, var_four)
Arguments identation is mixed - see https://peps.python.org/pep-0008/#indentation

```

Block scope rules:

```rust
// # AIRULE: enum names should be one-word only
enum Cake {
 Frosting,
 Icing,
 Coating
 GreatCoating
}
```

Running melange on this file would output:

```bash
melange: xxx.rs:5 - rule "enum names should be one-word only" violation
enum Cake {
 Frosting,
 Icing,
 Coating
 GreatCoating
}
```

## Configuration

The llm provider and the relevant settings are configured through the melange-config.toml file.
Melange supports both local providers such as ollama or remote ones such as openai, google, etc.
The api keys, where necessary, are looked up in the environment under the key `PROVIDER_API_KEY` (e.g. OPENAI_API_KEY).

Example configuration:

```bash
> cat melange-config.toml
provider = "ollama"
model = "qwen2.5-coder:0.5b"
max_tokens = 4096
```
