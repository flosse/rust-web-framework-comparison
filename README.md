# Rust web framework comparison

A comparison of some web frameworks written in [Rust](https://rustlang.org).

At the moment there are four interesting framworks to build web applications
with rust:

- **iron**     ([homepage](http://ironframework.io/) / [repository](https://github.com/iron/iron/)            / [documentation](http://ironframework.io/doc/iron/))
- **nickel**   ([homepage](http://nickel.rs/)        / [repository](https://github.com/nickel-org/nickel.rs/) / [documentation](http://docs.nickel.rs/nickel/))
- **rustful**  ( -                                   / [repository](https://github.com/Ogeon/rustful)         / [documentation](http://ogeon.github.io/docs/rustful/master/rustful/))
- **rustless** ([homepage](http://rustless.org/)     / [repository](https://github.com/rustless/rustless)     / [documentation](http://rustless.org/rustless/doc/rustless/))

If you need a more low level control you can choose between three libraries:

- **hyper**     ([homepage](http://hyper.rs/) / [repository](https://github.com/hyperium/hyper)      / [documentation](http://hyper.rs/hyper/hyper/))
- **tiny-http** ( -                           / [repository](https://github.com/frewsxcv/tiny-http)) / [documentation](http://frewsxcv.github.io/tiny-http/tiny_http/index.html))
- **solicit**   ( -                           / [repository](https://github.com/mlalic/solicit)      / [documentation](https://mlalic.github.io/solicit/solicit/index.html))

To build web clients with Rust, you can chosse between three libraries:

- **hyper**
- **ease**    (- / [repository](https://github.com/SimonPersson/ease)       / [documentation](http://simonpersson.github.io/ease/))
- **jsonrpc** (- / [repository](https://github.com/apoelstra/rust-jsonrpc/) / [documentation](https://www.wpsoftware.net/rustdoc/jsonrpc/))


|                  Name | iron                                  | nickel                                                | rustful   | rustless  | hyper  | tiny-http  | solicit | ease  | jsonrpc  |
| --------------------- | ------------------------------------- | ----------------------------------------------------- | --------- | --------- |------- | ---------- | ------- | ----- | -------  |
|               License | MIT                                   | MIT                                                   | MIT       | MIT       | MIT    | Apache 2.0 | MIT     | MIT   | CC0-1.0  |
|          Github Stars | 1.8k                                  | 1k                                                    | 0.5k      | 0.1k      | 1.0k   | 0.1k       | 0.1k    | 0.05k | 0k       |
|          Contributors | 41                                    | 35                                                    | 6         | 7         | 78     | 6          | 2       | 2     | 1        |
|                Server | yes                                   | yes                                                   | yes       | yes       | yes    | yes        | yes     | no    | no       |
|                Client | ?                                     | ?                                                     | ?         | ?         | ?      | ?          | yes     | yes   | yes      |
|        Base framework | hyper                                 | hyper                                                 | hyper     | iron      | yes    | -          | -       | hyper | hyper    |
|         HTTPS support | yes                                   | no                                                    | yes       | ?         | yes    | no         | -       | -     | -        |
         HTTP/2 support | ?                                     | ?                                                     | ?         | ?         | ?      | ?          | yes     | ?     | ?        |
|   Static File Serving | [yes](https://github.com/iron/static) | yes                                                   | ?         | ?         | -      | -          | -       | -     | -        |
|     Logger middleware | [yes](https://github.com/iron/logger) | no                                                    | ?         | ?         | -      | -          | -       | -     | -        |
| PostgreSQL middleware | ?                                     | [yes](https://github.com/nickel-org/nickel-postgres)  | ?         | ?         | -      | -          | -       | -     | -        |
|     SQLite middleware | ?                                     | [yes](https://github.com/flosse/nickel-sqlite)        | ?         | ?         | -      | -          | -       | -     | -        |

## Examples

To compile or run the examples use [Cargo](https://github.com/rust-lang/cargo).
First clone this repo

    git clone https://github.com/flosse/rust-web-framework-comparison
    cd rust-web-framework-comparison/

and change to the desired frameworkd directory (e.g. `cd iron/`) and type

    cargo run --example hello_world

Then visit `http://localhost:3000` to see the result.
