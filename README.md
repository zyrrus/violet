# Violet

TODO: Language description

    ---
    Here is a brief sample of the
    language and its syntax
    ---

    -- Import statements
    import std.collections {
        Heap,
        HashMap,
    }

    -- Function declaration with types
    fn fizz-buzz(i: Num) -> Str {
        --- This is technically a legal comment ---
        let fb = "";

        if i % 3 == 0 {
            fb = fb ++ "fizz";
        }

        if i % 5 == 0 {
            fb ++= "buzz";
        }

        return i if Str.is-empty(fb) else fb;
    }

    -- Function declaration without types
    fn main() {
        const fb: Str = fizz-buzz();
    }

## Interesting ideas

-   Identifiers can include unconventional characters like `-`
    -   NOTE: This likely means that whitespace is required between normal operators (ex: require `x - y` over `x-y`)
-   Custom operator definitions

## Syntax

### Comments

    -- for single-line comments

    ---
    for multi-line comments
    ---

### Operators

#### Arithmetic:

Classic: `+, -, /, *`  
Integer division: `//`

#### Logical

`and, or, not`

#### Bit-wise

#### Misc

Array concatenation: `++`  
Array contains: `in`

## Keywords

fn
let/const
