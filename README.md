# Rust web framework comparison

A comparison of some web frameworks written in [Rust](https://rustlang.org).

At the moment there are four interesting framworks to build web applications
with rust:

- iron     ([homepage](http://ironframework.io/) / [repository](https://github.com/iron/iron/)            / [documentation](http://ironframework.io/doc/iron/)                   )
- nickel   ([homepage](http://nickel.rs/)        / [Repository](https://github.com/nickel-org/nickel.rs/) / [documentation](http://docs.nickel.rs/nickel/)                       )
- rustful  ( -                                   / [Repository](https://github.com/Ogeon/rustful)         / [documentation](http://ogeon.github.io/docs/rustful/master/rustful/) )
- rustless ([homepage](http://rustless.org/)     / [Repository](https://github.com/rustless/rustless)     / [documentation](http://rustless.org/rustless/doc/rustless/)          )


|                  Name | iron                                  | nickel                                                | rustful   | rustless  |
| --------------------- | ------------------------------------- | ----------------------------------------------------- | --------- | --------- |
|               License | MIT                                   | MIT                                                   | MIT       | MIT       |
|          Github Stars | 1.8k                                  | 1k                                                    | 0.5k      | 0.1k      |
|          Contributors | 41                                    | 35                                                    | 6         | 7         |
|        Base framework | hyper                                 | hyper                                                 | hyper     | iron      |
|         HTTPS support | yes                                   | no                                                    | yes       | ?         |
|   Static File Serving | [yes](https://github.com/iron/static) | yes                                                   | ?         | ?         |
|     Logger middleware | [yes](https://github.com/iron/logger) | no                                                    | ?         | ?         |
| PostgreSQL middleware | ?                                     | [yes](https://github.com/nickel-org/nickel-postgres)  | ?         | ?         |
|     SQLite middleware | ?                                     | [yes](https://github.com/flosse/nickel-sqlite)        | ?         | ?         |
