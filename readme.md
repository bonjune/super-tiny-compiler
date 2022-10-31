# Super Tiny Compiler

This toy project will contain

1. `Tokenizer` (a.k.a lexer,) which takes an string input and produces a series of `Token`s.
2. `AstBuilder`, which transforms a series of `Token`s into a single `Expr` (astract syntax tree).
3. `Interpreter`, which evaluates a given `Expr` into a value.
4. `IrGen`, which converts `Expr` into an intermediate representation (could be LLVM IR).

## Language Specification

```
<digits> ::= "0" | "1" | ... | "9"
<num> ::= <digits>
        | <digits> <num>
<expr> ::=  <num>
        |   <expr> "+" <expr>
```

And the semantics is straight-forward.

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

`AstBuilder` consumes `Tokenizer` and builds an abstract syntax tree, which is `Expr`.

## `Interpreter`

`Interpreter` evaluates `Expr` into `Value`. With an interpreter, we can test working example of `Expr`!

## `IrGen`

Nothing fixed.
