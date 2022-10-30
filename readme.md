# Super Tiny Compiler

## Language Specification

```
<digits> ::= "0" | "1" | ... | "9"
<num> ::= <digits>
        | <digits> <num>
<expr> ::=  <num>
        |   <expr> "+" <expr>
```

## Tokenizer

`Tokenizer` is an `Iterator` which traverses string slice, producing `Token` values.

### Rust's `String` is UTF-8

There are confusing concepts and facts on what string actually is.
In many programming languages, such details are abstracted away so that a programmer does not deal with how string works internally.
But, in Rust, those details are exposed to a programmer with it's type system and APIs, which makes programmers fully understand string.

#### Keypoints

* UTF-8
  * UTF-8 is a variable-length encoding. An UTF-8 character consists of 1 to 4 bytes.
  * https://en.wikipedia.org/wiki/UTF-8
* Rust's `String` and `char`
  * `char` is of 4 bytes, for an UTF-8 character can take up to 4 bytes. This is a huge difference from C.
  * `String` and `str` are UTF-8 encoded strings, and series of contiguous bytes. An array of `char`s is not identical to a `String` of same UTF-8 characters.

## `AstBuilder`

`AstBuilder` consumes `Tokenizer` and builds an abstract syntax tree.

## CodeGen


