# Rust web framework comparison

A comparison of some web frameworks written in [Rust](https://www.rust-lang.org).

## Table of Contents

- [Server frameworks](#server-frameworks)
  - [Outdated server frameworks](#outdated-server-frameworks)
- [Client frameworks](#client-frameworks)
  - [Outdated client frameworks](#outdated-client-frameworks)
- [Frontend frameworks (WASM)](#frontend-frameworks-wasm)
- [Supplemental libraries](#supplemental-libraries)
  - [Websocket](#websocket)
  - [Templating](#templating)
- [Comparison](#comparison)
  - [High-Level Server Frameworks](#high-level-server-frameworks)
  - [Low-Level Frameworks](#low-level-frameworks)
  - [Frontend Frameworks](#frontend-frameworks)
  - [Middleware & Plugins](#middleware--plugins)
  - [Websocket Libraries](#websocket-libraries)
- [Resources](#resources)
  - [Blog posts](#blog-posts)
  - [Demos](#demos)
  - [Real-world web projects using Rust](#real-world-web-projects-using-rust)
  - [JS & asm.js & WASM](#js--asmjs--wasm)

## Server frameworks

There are several interesting frameworks to build web applications with Rust:

- **actix-web** ([homepage](https://actix.rs/)            / [repository](https://github.com/actix/actix-web)       / [documentation](https://actix.github.io/actix-web/actix_web/) / [user guide](https://actix.rs/docs/))
- **gotham**    ([homepage](http://gotham.rs/)            / [repository](https://github.com/gotham-rs/gotham/)     / [documentation](https://docs.rs/gotham/) / [examples](https://github.com/gotham-rs/gotham/tree/master/examples))
- **iron**      ([homepage](http://ironframework.io/)     / [repository](https://github.com/iron/iron/)            / [documentation](http://ironframework.io/doc/iron/))
- **nickel**    ([homepage](http://nickel-org.github.io/) / [repository](https://github.com/nickel-org/nickel.rs/) / [documentation](https://docs.rs/nickel))
- **rocket**    ([homepage](https://rocket.rs/)           / [repository](https://github.com/SergioBenitez/rocket)  / [documentation](https://rocket.rs/guide/))
- **rouille**   ( -                                       / [repository](https://github.com/tomaka/rouille)        / [documentation](http://tomaka.github.io/rouille/rouille/index.html))
- **Thruster**  ( -                                       / [repository](https://github.com/trezm/Thruster)        / [documentation](https://docs.rs/thruster) / [examples](https://github.com/trezm/Thruster/tree/master/examples))
- **Tide**      ( -                                       / [repository](https://github.com/rustasync/tide)        / [documentation](https://docs.rs/tide) / [examples](https://github.com/rustasync/tide/tree/master/examples))
- **tower-web** ( -                                       / [repository](https://github.com/carllerche/tower-web)  / [documentation](https://docs.rs/tower-web/) / [examples](https://github.com/carllerche/tower-web/tree/master/examples))
- **warp**      ( -                                       / [repository](https://github.com/seanmonstar/warp)      / [documentation](https://docs.rs/warp/) / [examples](https://github.com/seanmonstar/warp/tree/master/examples))

If you need a more low level control you can choose between these libraries:

- **hyper**     ([homepage](http://hyper.rs/) / [repository](https://github.com/hyperium/hyper)       / [documentation](http://hyper.rs/hyper))
- **tiny-http** ( -                           / [repository](https://github.com/frewsxcv/tiny-http))  / [documentation](http://frewsxcv.github.io/tiny-http/tiny_http/index.html))
- **h2**        ( -                           / [repository](https://github.com/carllerche/h2)        / - )

### Outdated server frameworks

- [civet](https://github.com/wycats/rust-civet)
- [conduit](https://github.com/conduit-rust/conduit)
- [cargonaouts](https://github.com/cargonauts-rs/cargonauts)
- [hydrogen](https://github.com/nathansizemore/hydrogen)
- [kinglet](https://github.com/pyfisch/kinglet)
- [rustless](https://github.com/rustless/rustless)
- [rustful](https://github.com/Ogeon/rustful)
- [shio](https://github.com/mehcode/shio-rs)
- [sappers](https://github.com/sappworks/sapper)
- [solicit](https://github.com/mlalic/solicit)
- [tk-http](https://github.com/swindon-rs/tk-http)

## Client frameworks

To build web clients with Rust, you can choose between these libraries:

- **actix-web**  ([homepage](https://actix.rs/) / [repository](https://github.com/actix/actix-web)         / [api docs](https://actix.github.io/actix-web/actix_web/client/index.html))
- **reqwest**    (-                             / [repository](https://github.com/seanmonstar/reqwest)     / [documentation](https://docs.rs/reqwest))
- **hyper**      ([homepage](http://hyper.rs/)  / [repository](https://github.com/hyperium/hyper)          / [documentation](http://hyper.rs/hyper/hyper/))
- **jsonrpc**    (-                             / [repository](https://github.com/apoelstra/rust-jsonrpc/) / [documentation](https://www.wpsoftware.net/rustdoc/jsonrpc/))

### Outdated client frameworks

- [ease](https://github.com/SimonPersson/ease)

## Frontend frameworks (WASM)

Since [WASM](http://webassembly.org/) support is available in most browsers we can use Rust to build web applications :)

- **stdweb** ( - / [repository](https://github.com/koute/stdweb) / [documentation](https://docs.rs/stdweb/) ) A standard library for the client-side Web
- **yew**    ( [homepage](https://yew.rs)  / [repository](https://github.com/yewstack/yew) / [documentation](https://docs.rs/yew/) ) - A frontend framework inspired by Elm and React (based on stdweb)
- **percy**  ( [homepage](https://chinedufn.github.io/percy/) / [repository](https://github.com/chinedufn/percy) / - ) - A modular toolkit for building isomorphic web apps
- **seed**   ( [homepage](http://seed-rs.org/) / [repository](https://github.com/David-OConnor/seed) / - ) - A Rust framework for creating web apps
- **draco**  ( - / [repository](https://github.com/utkarshkukreti/draco) / [documentation](https://docs.rs/draco/) ) - A frontend framework inpired by Redux and Elm
- **smithy** ([homepage](https://www.smithy.rs) - / [repository](https://github.com/rbalicki2/smithy) / - [documentation](https://docs.smithy.rs/smithy/) ) - A front-end framework
- **squark** ( - / [repository](https://github.com/rail44/squark) / [documentation](https://docs.rs/squark) ) - Rust frontend framework, for web browser and more.
- **willow** ( [homepage](http://sindrejohansen.no/willow/) - / [repository](https://github.com/sindreij/willow) / - ) - A frontend framework inspired by Elm
- **dodrio** ( - / [repository](https://github.com/fitzgen/dodrio) / [documentation](https://docs.rs/dodrio/) ) - A fast, bump-allocated virtual DOM library.
- **dominator** ( - / [repository](https://github.com/Pauan/rust-dominator) / [documentation](https://docs.rs/dominator/) - Zero cost declarative DOM library using FRP signals for Rust!.
- **mika** ( [homepage](https://limira-rs.gitlab.io/mika/) / [repository](https://gitlab.com/limira-rs/mika) / - ) - A signal-based framework for building front-end app, it tries to help, but may cause annoyances.
- **mogwai** ( - / [repository](https://github.com/schell/mogwai) / [documentation](https://docs.rs/mogwai/) ) - The minimalist, obvious, graphical, web application interface

### Outdated frontend frameworks

- [ruukh](https://github.com/csharad/ruukh)
- **sauron** ( - / [repository](https://github.com/ivanceras/sauron) / [documentation](https://docs.rs/sauron/ ) - Sauron is an html web framework for building web-apps. It is heavily inspired by elm.

## Supplemental libraries

### Websocket

- **websocket**   ([homepage](https://websockets-rs.github.io/rust-websocket/)   / [repository](https://github.com/cyderize/rust-websocket)  / [documentation](https://websockets-rs.github.io/rust-websocket/doc/websocket/))
- **ws-rs**       ([homepage](https://ws-rs.org)                           / [repository](https://github.com/housleyjk/ws-rs)          / [documentation](https://ws-rs.org/docs))
- **tungstenite** ( -                                                      / [repository](https://github.com/snapview/tungstenite-rs)  / [documentation](https://docs.rs/crate/tungstenite/))
- **actix-web**   ([homepage](https://actix.rs/)                           / [repository](https://github.com/actix/actix-web)          / [documentation](https://actix.github.io/actix-web/actix_web/))

### Templating

- **tera**       ([homepage](https://tera.netlify.com/)                 / [repository](https://github.com/Keats/tera)               / [documentation](https://docs.rs/tera/))
- **mustache**   (-                                                     / [repository](https://github.com/nickel-org/rust-mustache) / [documentation](http://nickel-org.github.io/rust-mustache))
- **liquid**     (-                                                     / [repository](https://github.com/cobalt-org/liquid-rust)   / - )
- **handlebars** (-                                                     / [repository](https://github.com/sunng87/handlebars-rust)  / [documentation](https://docs.rs/crate/handlebars/))
- **horrorshow** (-                                                     / [repository](https://github.com/Stebalien/horrorshow-rs)  / [documentation](https://stebalien.github.io/horrorshow-rs/horrorshow/))
- **maud**       ([homepage](https://lfairy.gitbooks.io/maud/content/)  / [repository](https://github.com/lfairy/maud)              / [documentation](https://lambda.xyz/maud/maud/))
- **askama**     (-                                                     / [repository](https://github.com/djc/askama)               / [documentation](https://docs.rs/askama/) )
- **stpl**       (-                                                     / [repository](https://github.com/dpc/stpl)                 / - )
- **ructe**      (-                                                     / [repository](https://github.com/kaj/ructe)                / [documentation](https://docs.rs/ructe/) )
- **typed-html** (-                                                     / [repository](https://github.com/bodil/typed-html)         / [documentation](https://docs.rs/typed-html/) )

## Comparison

### High-Level Server Frameworks

| Name               | rocket                                                                                                 | warp                                                                                                   | iron                                                                                      | actix-web                                                                                            | nickel                                                                                                 | gotham                                                                                             | rouille                                                                                           | Thruster                                                                                           | tide                                                                                         |
| ---                | ---                                                                                                    | ---                                                                                                    | ---                                                                                       | ---                                                                                                  | ---                                                                                                    | ---                                                                                                | ---                                                                                               | ---                                                                                                | ---                                                                                          |
| **License**        | ![Rocket license](https://img.shields.io/crates/l/rocket.svg?label=%20)                                | ![warp license](https://img.shields.io/crates/l/warp.svg?label=%20)                                    | ![Iron license](https://img.shields.io/crates/l/iron.svg?label=%20)                       | ![Actix-web license](https://img.shields.io/crates/l/actix-web.svg?label=%20)                        | ![Nickel license](https://img.shields.io/crates/l/nickel.svg?label=%20)                                | ![Gotham license](https://img.shields.io/crates/l/gotham.svg?label=%20)                            | ![Rouille license](https://img.shields.io/crates/l/rouille.svg?label=%20)                         | ![Thruster license](https://img.shields.io/crates/l/thruster.svg?label=%20)                        | ![Tide license](https://img.shields.io/crates/l/tide.svg?label=%20)                          |
| **Version**        | ![Rocket version](https://img.shields.io/crates/v/rocket.svg?label=%20)                                | ![warp version](https://img.shields.io/crates/v/warp.svg?label=%20)                                    | ![Iron version](https://img.shields.io/crates/v/iron.svg?label=%20)                       | ![Actix-web version](https://img.shields.io/crates/v/actix-web.svg?label=%20)                        | ![Nickel version](https://img.shields.io/crates/v/nickel.svg?label=%20)                                | ![Gotham version](https://img.shields.io/crates/v/gotham.svg?label=%20)                            | ![Rouille version](https://img.shields.io/crates/v/rouille.svg?label=%20)                         | ![Thruster version](https://img.shields.io/crates/v/thruster.svg?label=%20)                        | ![Tide version](https://img.shields.io/crates/v/tide.svg?label=%20)                          |
| **Github Stars**   | ![Rocket stars](https://img.shields.io/github/stars/SergioBenitez/Rocket.svg?label=%20)                | ![warp stars](https://img.shields.io/github/stars/seanmonstar/warp.svg?label=%20)                      | ![Iron stars](https://img.shields.io/github/stars/iron/iron.svg?label=%20)                | ![Actix-web stars](https://img.shields.io/github/stars/actix/actix-web.svg?label=%20)                | ![Nickel stars](https://img.shields.io/github/stars/nickel-org/nickel.rs.svg?label=%20)                | ![Gotham stars](https://img.shields.io/github/stars/gotham-rs/gotham.svg?label=%20)                | ![Rouille stars](https://img.shields.io/github/stars/tomaka/rouille.svg?label=%20)                | ![Thruster stars](https://img.shields.io/github/stars/trezm/Thruster.svg?label=%20)                | ![Tide stars](https://img.shields.io/github/stars/http-rs/tide.svg?label=%20)                |
| **Contributors**   | ![Rocket contributors](https://img.shields.io/github/contributors/SergioBenitez/Rocket.svg?label=%20)  | ![warp contributors](https://img.shields.io/github/contributors/seanmonstar/warp.svg?label=%20)        | ![Iron contributors](https://img.shields.io/github/contributors/iron/iron.svg?label=%20)  | ![Actix-web contributors](https://img.shields.io/github/contributors/actix/actix-web.svg?label=%20)  | ![Nickel contributors](https://img.shields.io/github/contributors/nickel-org/nickel.rs.svg?label=%20)  | ![Gotham contributors](https://img.shields.io/github/contributors/gotham-rs/gotham.svg?label=%20)  | ![Rouille contributors](https://img.shields.io/github/contributors/tomaka/rouille.svg?label=%20)  | ![Thruster contributors](https://img.shields.io/github/contributors/trezm/Thruster.svg?label=%20)  | ![Tide contributors](https://img.shields.io/github/contributors/http-rs/tide.svg?label=%20)  |
| **Activity**       | ![Rocket activity](https://img.shields.io/github/commit-activity/y/SergioBenitez/Rocket.svg?label=%20) | ![warp activity](https://img.shields.io/github/commit-activity/y/seanmonstar/warp.svg?label=%20)       | ![Iron activity](https://img.shields.io/github/commit-activity/y/iron/iron.svg?label=%20) | ![Actix-web activity](https://img.shields.io/github/commit-activity/y/actix/actix-web.svg?label=%20) | ![Nickel activity](https://img.shields.io/github/commit-activity/y/nickel-org/nickel.rs.svg?label=%20) | ![Gotham activity](https://img.shields.io/github/commit-activity/y/gotham-rs/gotham.svg?label=%20) | ![Rouille activity](https://img.shields.io/github/commit-activity/y/tomaka/rouille.svg?label=%20) | ![Thruster activity](https://img.shields.io/github/commit-activity/y/trezm/Thruster.svg?label=%20) | ![Tide activity](https://img.shields.io/github/commit-activity/y/http-rs/tide.svg?label=%20) |
| **Base framework** | hyper                                                                                                  | hyper                                                                                                  | hyper                                                                                     | tokio                                                                                                | hyper                                                                                                  | hyper                                                                                              | tiny-http                                                                                         | tokio (or hyper)                                                                                   | hyper                                                                                        |
| **HTTPS support**  |                                                                                                        |                                                                                                        | yes                                                                                       | yes                                                                                                  | no                                                                                                     | yes                                                                                                | ?                                                                                                 | yes                                                                                                |                                                                                              |
| **HTTP/2 support** |                                                                                                        |                                                                                                        | ?                                                                                         | yes                                                                                                  | ?                                                                                                      | no                                                                                                 | ?                                                                                                 | yes                                                                                                |                                                                                              |
| **Async**          | no                                                                                                     | yes                                                                                                    |                                                                                           | yes                                                                                                  |                                                                                                        | yes                                                                                                |                                                                                                   | yes                                                                                                | yes                                                                                          |
| **Stable Rust**    | no																																																			| yes																																																		 |																																													 | yes																																																	|                                                                                                        | yes                                                                                                   |                                                                                                   | yes                                                                                                |                                                                                              |

### Low-Level Frameworks

| Name               | hyper                                                                                           | h2                                                                                          | tiny-http                                                                                                |
| ---                | ---                                                                                             | ---                                                                                         | ---                                                                                                      |
| **License**        | ![Hyper license](https://img.shields.io/crates/l/hyper.svg?label=%20)                           | ![H2 license](https://img.shields.io/crates/l/h2.svg?label=%20)                             | ![Tiny-http license](https://img.shields.io/crates/l/tiny-http.svg?label=%20)                            |
| **Version**        | ![Hyper version](https://img.shields.io/crates/v/hyper.svg?label=%20)                           | ![H2 version](https://img.shields.io/crates/v/h2.svg?label=%20)                             | ![Tiny-http version](https://img.shields.io/crates/v/tiny-http.svg?label=%20)                            |
| **Github Stars**   | ![Hyper stars](https://img.shields.io/github/stars/hyperium/hyper.svg?label=%20)                | ![H2 stars](https://img.shields.io/github/stars/carllerche/h2.svg?label=%20)                | ![Tiny-http stars](https://img.shields.io/github/stars/tiny-http/tiny-http.svg?label=%20)                |
| **Contributors**   | ![Hyper contributors](https://img.shields.io/github/contributors/hyperium/hyper.svg?label=%20)  | ![H2 contributors](https://img.shields.io/github/contributors/carllerche/h2.svg?label=%20)  | ![Tiny-http contributors](https://img.shields.io/github/contributors/tiny-http/tiny-http.svg?label=%20)  |
| **Activity**       | ![Hyper activity](https://img.shields.io/github/commit-activity/y/hyperium/hyper.svg?label=%20) | ![H2 activity](https://img.shields.io/github/commit-activity/y/carllerche/h2.svg?label=%20) | ![Tiny-http activity](https://img.shields.io/github/commit-activity/y/tiny-http/tiny-http.svg?label=%20) |
| **Server**         | yes                                                                                             | yes                                                                                         | yes                                                                                                      |
| **Client**         | yes                                                                                             | yes                                                                                         | ?                                                                                                        |
| **HTTPS support**  | yes                                                                                             | no                                                                                          | yes                                                                                                      |
| **HTTP/2 support** | solicit                                                                                         | yes                                                                                         | ?                                                                                                        |
| **Async**          | yes                                                                                             | yes                                                                                         |                                                                                                          |


### Frontend Frameworks

| Name               | yew                                                                                             | stdweb                                                                                         | percy                                                                                            | dodrio                                                                                           | seed                                                                                               | draco                                                                                                 | squark                                                                                          | willow                                                                                            | smithy                                                                                             | dominator                                                                                                 | mogwai                                                                                          |
| ---                | ---                                                                                             | ---                                                                                            | ---                                                                                              | ---                                                                                              | ---                                                                                                | ---                                                                                                   | ---                                                                                             | ---                                                                                               | ---                                                                                                | ---                                                                                                       | ---                                                                                             |
| **License**        | ![Yew license](https://img.shields.io/crates/l/yew.svg?label=%20)                               | ![Stdweb license](https://img.shields.io/crates/l/stdweb.svg?label=%20)                        | ![Percy license](https://img.shields.io/crates/l/percy.svg?label=%20)                            | ![Dodrio license](https://img.shields.io/crates/l/dodrio.svg?label=%20)                          | ![Seed license](https://img.shields.io/crates/l/seed.svg?label=%20)                                | ![Draco license](https://img.shields.io/crates/l/draco.svg?label=%20)                                 | ![Squark license](https://img.shields.io/crates/l/squark.svg?label=%20)                         | ![Willow license](https://img.shields.io/github/license/sindreij/willow.svg?label=%20)            | ![Smithy license](https://img.shields.io/crates/l/smithy.svg?label=%20)                            | ![Dominator license](https://img.shields.io/crates/l/dominator.svg?label=%20)                             | ![Mogwai license](https://img.shields.io/crates/l/mogwai.svg?label=%20)                         |
| **Version**        | ![Yew version](https://img.shields.io/crates/v/yew.svg?label=%20)                               | ![Stdweb version](https://img.shields.io/crates/v/stdweb.svg?label=%20)                        | ![Percy version](https://img.shields.io/crates/v/percy.svg?label=%20)                            | ![Dodrio version](https://img.shields.io/crates/v/dodrio.svg?label=%20)                          | ![Seed version](https://img.shields.io/crates/v/seed.svg?label=%20)                                | ![Draco version](https://img.shields.io/crates/v/draco.svg?label=%20)                                 | ![Squark version](https://img.shields.io/crates/v/squark.svg?label=%20)                         | no                                                                                                | ![Smithy version](https://img.shields.io/crates/v/smithy.svg?label=%20)                            | ![Dominator version](https://img.shields.io/crates/v/dominator.svg?label=%20)                             | ![Mogwai version](https://img.shields.io/crates/v/mogwai.svg?label=%20)                         |
| **Github Stars**   | ![Yew stars](https://img.shields.io/github/stars/yewstack/yew.svg?label=%20)                    | ![Stdweb stars](https://img.shields.io/github/stars/koute/stdweb.svg?label=%20)                | ![Percy stars](https://img.shields.io/github/stars/chinedufn/percy.svg?label=%20)                | ![Dodrio stars](https://img.shields.io/github/stars/fitzgen/dodrio.svg?label=%20)                | ![Seed stars](https://img.shields.io/github/stars/David-OConnor/seed.svg?label=%20)                | ![Draco stars](https://img.shields.io/github/stars/utkarshkukreti/draco.svg?label=%20)                | ![Squark stars](https://img.shields.io/github/stars/rail44/squark.svg?label=%20)                | ![Willow stars](https://img.shields.io/github/stars/sindreij/willow.svg?label=%20)                | ![Smithy stars](https://img.shields.io/github/stars/rbalicki2/smithy.svg?label=%20)                | ![Dominator stars](https://img.shields.io/github/stars/Pauan/rust-dominator.svg?label=%20)                | ![Mogwai stars](https://img.shields.io/github/stars/schell/mogwai.svg?label=%20)                |
| **Contributors**   | ![Yew contributors](https://img.shields.io/github/contributors/yewstack/yew.svg?label=%20)      | ![Stdweb contributors](https://img.shields.io/github/contributors/koute/stdweb.svg?label=%20)  | ![Percy contributors](https://img.shields.io/github/contributors/chinedufn/percy.svg?label=%20)  | ![Dodrio contributors](https://img.shields.io/github/contributors/fitzgen/dodrio.svg?label=%20)  | ![Seed contributors](https://img.shields.io/github/contributors/David-OConnor/seed.svg?label=%20)  | ![Draco contributors](https://img.shields.io/github/contributors/utkarshkukreti/draco.svg?label=%20)  | ![Squark contributors](https://img.shields.io/github/contributors/rail44/squark.svg?label=%20)  | ![Willow contributors](https://img.shields.io/github/contributors/sindreij/willow.svg?label=%20)  | ![Smithy contributors](https://img.shields.io/github/contributors/rbalicki2/smithy.svg?label=%20)  | ![Dominator contributors](https://img.shields.io/github/contributors/Pauan/rust-dominator.svg?label=%20)  | ![Mogwai contributors](https://img.shields.io/github/contributors/schell/mogwai.svg?label=%20)  |
| **Activity**       | ![Yew activity](https://img.shields.io/github/commit-activity/y/yewstack/yew.svg?label=%20)     | ![Stdweb activity](https://img.shields.io/github/commit-activity/y/koute/stdweb.svg?label=%20) | ![Percy activity](https://img.shields.io/github/commit-activity/y/chinedufn/percy.svg?label=%20) | ![Dodrio activity](https://img.shields.io/github/commit-activity/y/fitzgen/dodrio.svg?label=%20) | ![Seed activity](https://img.shields.io/github/commit-activity/y/David-OConnor/seed.svg?label=%20) | ![Draco activity](https://img.shields.io/github/commit-activity/y/utkarshkukreti/draco.svg?label=%20) | ![Squark activity](https://img.shields.io/github/commit-activity/y/rail44/squark.svg?label=%20) | ![Willow activity](https://img.shields.io/github/commit-activity/y/sindreij/willow.svg?label=%20) | ![Smithy activity](https://img.shields.io/github/commit-activity/y/rbalicki2/smithy.svg?label=%20) | ![Dominator activity](https://img.shields.io/github/commit-activity/y/Pauan/rust-dominator.svg?label=%20) | ![Mogwai activity](https://img.shields.io/github/commit-activity/y/schell/mogwai.svg?label=%20) |
| **Stable Rust**    | **yes**                                                                                         | **yes**                                                                                        | no                                                                                               | ?                                                                                                | **yes**                                                                                            | **yes**                                                                                               | no                                                                                              | no                                                                                                | no                                                                                                 | no                                                                                                        | no                                                                                              |
| **Base framework** | wasm-bindgen (or stdweb)                                                                        | -                                                                                              | wasm-bindgen                                                                                     | wasm-bindgen                                                                                     | wasm-bindgen                                                                                       | wasm-bindgen                                                                                          | wasm-bindgen                                                                                    | wasm-bindgen                                                                                      | wasm-bindgen                                                                                       | wasm-bindgen                                                                                              | wasm-bindgen                                                                                    |
| **Virtual DOM**    | yes                                                                                             | ?                                                                                              | yes                                                                                              | yes                                                                                              | yes                                                                                                | yes                                                                                                   | yes                                                                                             | ?                                                                                                 | yes                                                                                                | **no**                                                                                                    | **no**                                                                                          |

### Middleware & Plugins

|           Name            |                    iron                    | gotham |                        nickel                         | rouille | actix-web                                                                                     | Thruster
| ------------------------- | ------------------------------------------ | ------ | ----------------------------------------------------- | ------- | --------------------------------------------------------------------------------------------- |-----
| **Static File Serving**   | [yes](https://github.com/iron/staticfile)  | no^    | yes                                                   | n/a     | [yes](https://actix.rs/docs/static-files/)                                                    | [yes](https://github.com/thruster-rs/Thruster/tree/master/thruster/examples/static_file)
| **Mounting**              | [yes](https://github.com/iron/mount)       | yes    | yes                                                   | n/a     | [yes](https://actix.rs/docs/application/#using-an-application-prefix-to-compose-applications) | no
| **Logging**               | [yes](https://github.com/iron/logger)      | yes    | no                                                    | n/a     | [yes](https://actix.rs/docs/middleware/#logging)																							| [yes](https://github.com/thruster-rs/Thruster/blob/master/thruster/examples/profiling.rs)
| **JSON-Body-Parsing**     | [yes](https://github.com/iron/body-parser) | yes    | yes                                                   | n/a     | [yes](https://github.com/actix/examples/tree/master/json)																			| yes (via Serde)
| **Sessions**              | [yes](https://github.com/iron/session)     | yes    | ?                                                     | n/a     | [yes](https://actix.rs/docs/middleware/#user-sessions)																				| [yes](https://github.com/thruster-rs/Thruster/tree/master/thruster/examples/jwt_example)
| **Cookies**               | [yes](https://github.com/iron/cookie)      | yes    | ?                                                     | n/a     | [yes](https://actix.rs/docs/middleware/#user-sessions)																				| [yes](https://github.com/thruster-rs/Thruster/blob/master/thruster/src/middleware/cookies.rs)
| **PostgreSQL middleware** | ?                                          | no^    | [yes](https://github.com/nickel-org/nickel-postgres)  | n/a     | [yes](https://github.com/actix/examples/tree/master/diesel)																		|
| **SQLite middleware**     | ?                                          | no^    | [yes](https://github.com/flosse/nickel-sqlite)        | n/a     | [yes](https://github.com/actix/examples/tree/master/diesel)																		|
| **Redis middleware**      | ?                                          | no^    | [yes](https://github.com/matthewbentley/nickel-redis) | n/a     | [yes](https://github.com/actix/actix-redis)																										|
| **MySQL middleware**      | ?                                          | no^    | [yes](https://github.com/zither/nickel-mysql)         | n/a     | [yes](https://github.com/actix/examples/tree/master/diesel)																		|

(^ Planned in current roadmap)

### Websocket Libraries

| Name               | websocket                                                                                                         | ws-rs                                                                                            | twist                                                                                             | tungstenite                                                                                                    | actix-web                                                                                            |
| ---                | ---                                                                                                               | ---                                                                                              | ---                                                                                               | ---                                                                                                            | ---                                                                                                  |
| **License**        | ![Websocket license](https://img.shields.io/crates/l/websocket.svg?label=%20)                                     | ![Ws-rs license](https://img.shields.io/crates/l/ws.svg?label=%20)                               | ![Twist license](https://img.shields.io/crates/l/ws.svg?label=%20)                                | ![Tungstenite license](https://img.shields.io/crates/l/tungstenite.svg?label=%20)                              | ![Actix-web license](https://img.shields.io/crates/l/actix-web.svg?label=%20)                        |
| **Version**        | ![Websocket version](https://img.shields.io/crates/v/websocket.svg?label=%20)                                     | ![Ws-rs version](https://img.shields.io/crates/v/ws.svg?label=%20)                               | ![Twist version](https://img.shields.io/crates/v/ws.svg?label=%20)                                | ![Tungstenite version](https://img.shields.io/crates/v/tungstenite.svg?label=%20)                              | ![Actix-web version](https://img.shields.io/crates/v/actix-web.svg?label=%20)                        |
| **Github Stars**   | ![Websocket stars](https://img.shields.io/github/stars/websockets-rs/rust-websocket.svg?label=%20)                | ![Ws-rs stars](https://img.shields.io/github/stars/housleyjk/ws-rs.svg?label=%20)                | ![Twist stars](https://img.shields.io/github/stars/rustyhorde/twist.svg?label=%20)                | ![Tungstenite stars](https://img.shields.io/github/stars/snapview/tungstenite-rs.svg?label=%20)                | ![Actix-web stars](https://img.shields.io/github/stars/actix/actix-web.svg?label=%20)                |
| **Contributors**   | ![Websocket contributors](https://img.shields.io/github/contributors/websockets-rs/rust-websocket.svg?label=%20)  | ![Ws-rs contributors](https://img.shields.io/github/contributors/housleyjk/ws-rs.svg?label=%20)  | ![Twist contributors](https://img.shields.io/github/contributors/rustyhorde/twist.svg?label=%20)  | ![Tungstenite contributors](https://img.shields.io/github/contributors/snapview/tungstenite-rs.svg?label=%20)  | ![Actix-web contributors](https://img.shields.io/github/contributors/actix/actix-web.svg?label=%20)  |
| **Activity**       | ![Websocket activity](https://img.shields.io/github/commit-activity/y/websockets-rs/rust-websocket.svg?label=%20) | ![Ws-rs activity](https://img.shields.io/github/commit-activity/y/housleyjk/ws-rs.svg?label=%20) | ![Twist activity](https://img.shields.io/github/commit-activity/y/rustyhorde/twist.svg?label=%20) | ![Tungstenite activity](https://img.shields.io/github/commit-activity/y/snapview/tungstenite-rs.svg?label=%20) | ![Actix-web activity](https://img.shields.io/github/commit-activity/y/actix/actix-web.svg?label=%20) |
| **Server**         | yes                                                                                                               | yes                                                                                              | yes                                                                                               | yes                                                                                                            | yes                                                                                                  |
| **Client**         | yes                                                                                                               | yes                                                                                              | yes                                                                                               | yes                                                                                                            | yes                                                                                                  |
| **Base framework** | - / tokio                                                                                                         | mio                                                                                              | tokio                                                                                             | - / tokio                                                                                                      | tokio                                                                                                |
| **Async**          | no / yes                                                                                                          | yes                                                                                              | yes                                                                                               | no / yes                                                                                                       | yes                                                                                                  |

## Resources

### Blog posts

#### 2018

- [Lessons learned on writing web applications completely in Rust](https://medium.com/@saschagrunert/lessons-learned-on-writing-web-applications-completely-in-rust-2080d0990287)
- [Introducing Ruukh Framework](https://sharadchand.com/2018/10/03/ruukh-framework.html)
- [Baby’s First Rust+WebAssembly module: Say hi to JSConf EU!](https://hacks.mozilla.org/2018/06/babys-first-rustwebassembly-module-say-hi-to-jsconf-eu/)
- [Mix Rust Code (WebAssembly) with Vue Component](https://busy.org/@drsensor/mix-rust-code-webassembly-with-vue-component-basic)
- [Wicked Fast Web Servers in Rust](https://medium.com/@MertzAlertz/wicked-fast-web-servers-in-rust-4947688426bc)
- [Migrating to Actix Web from Rocket for Stability](https://nbsoftsolutions.com/blog/migrating-to-actix-web-from-rocket-for-stability)
- [Creating a Rusty Rocket fuelled with Diesel](https://lankydanblog.com/2018/05/20/creating-a-rusty-rocket-fuelled-with-diesel/)

#### Until 2017

- [Dose Response ported to WebAssembly!](https://aimlesslygoingforward.com/blog/2017/12/25/dose-response-ported-to-webassembly/)
- [Rust and the case for WebAssembly in 2018](https://mgattozzi.com/rust-wasm)
- [wasm32-unknown-unknown landed & enabled](https://www.hellorust.com/news/native-wasm-target.html)
- [How to Deploy a Rocket Application to Heroku](http://www.duelinmarkers.com/2017/10/21/how-to-deploy-a-rocket-application-to-heroku.html)
- [Rust to WebAssembly, Made Easy](https://lord.io/blog/2017/wargo/)
- [Rust for the web](https://thefullsnack.com/en/rust-for-the-web.html)
- [Rocket on Fedora](https://sumantrom.blogspot.de/2017/04/rocket-on-fedora.html)
- [Announcing Gotham - A flexible web framework for stable Rust that does not sacrifice safety, security or speed.](https://gotham.rs/2017/08/09/announcing-gotham.html)
- [Announcing cargonauts - A Rust async web framework](https://medium.com/@withoutboats/announcing-cargonauts-db5efaaaf7d2)
- [Writing a GitHub webhook with Rust! Part 1: Rocket](https://medium.com/@aergonaut/writing-a-github-webhook-with-rust-part-1-rocket-4426dd06d45d)
- [Hello, Botket! (Rocket)](http://ootoovak.com/2017/04/08/botket/)
- [Launching a URL Shortener in Rust using Rocket](http://matthias-endler.de/2017/rust-url-shortener/)
- [Rocket + sodiumoxide = ♥](https://skinkade.github.io/rocket-encrypted-rest/)
- [The Path to Rust on the Web](http://asquera.de/blog/2017-04-10/the-path-to-rust-on-the-web/)
- [Rendering Vector Map Tiles (Rust + asm.js demo)](https://pyfisch.org/blog/rendering-vector-map-tiles/)
- [Compiling to the web with Rust and emscripten](https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627)
- [Basic 2FA in Rocket](https://skinkade.github.io/rocket-auth-demo/)
- [Building high performance REST APIs with Rust and Rocket](https://github.com/sebasmagri/rocket-loadavg-api/blob/master/README.md)
- [Rocket Rocks! Using FromFormValue Traits to protect your website](http://blog.stephanbuys.com/2017/01/rocket-rocks-using-fromformvalue-traits.html)
- [Building an Asynchronous Hyper Server](https://mgattozzi.com/hyper-async)
- [JWT & Access Roles in Rocket](https://skinkade.github.io/rocket-jwt-roles-demo/)
- [Writing a basic JSON response web server in Rust using Iron](https://www.jamestease.co.uk/blether/writing-a-basic-json-web-server-in-rust-using-iron)
- [Diesel Powered Rocket](https://mgattozzi.com/diesel-powered-rocket)
- [Using Stainless with Rocket](http://neikos.me/Using_Stainless_with_Rocket.html)
- [Integration testing a service written in Rust and Iron](https://www.nibor.org/blog/integration-testing-a-service-written-in-rust-and-iron/)
- [Actually using Iron: A grumpy introduction to web development in Rust](https://wiki.alopex.li/ActuallyUsingIron)
- [Using Rust for Webdev as a Hobby Programmer](http://neikos.me/Using_Rust_for_Webdev_as_a_Hobby_Programmer.html)
- [My adventures in Rust webdev](https://medium.com/@tomaka/my-adventures-in-rust-webdev-850c67be6c40)
- [Rust’s Iron Framework: First impressions](https://medium.com/@ericdreichert/rusts-iron-framework-first-impressions-dc8e2b1308e4)
- Rust for Node.js developers
    - [Part 3 - Crates, Modules and the web](http://fredrik.anderzon.se/rust-for-node-js-developers-part-3-crates-modules-and-the-web/)
    - [Part 2 - Can I borrow that?](http://fredrik.anderzon.se/2016/06/17/rust-for-node-js-developers-part-2-can-i-borrow-that/)
    - [Part 1 - Introduction to Rust](http://fredrik.anderzon.se/2016/05/10/rust-for-node-developers-part-1-introduction/)
- [A Rust-powered public web page in 5 minutes](https://medium.com/@rap2h/a-rust-powered-public-website-in-5-minutes-b682d8527b6b#.q0ehmcqim)
- [Rust and Rest](http://lucumr.pocoo.org/2016/7/10/rust-rest/)
- [Shipping forgettable microservices with Rust](https://precompile.com/2016/06/23/shipping-forgettable-microservices-with-rust.html)
- [Writing a simple REST app in Rust](https://gsquire.github.io/static/post/rest-in-rust/)
- [Getting started with Rust](https://tech.zalando.de/blog/getting-started-with-rust/)
- [Let's Build a Web Server in Rust](https://dfockler.github.io/2016/05/20/web-server.html)
- [Creating a basic webservice in Rust](http://hermanradtke.com/2016/05/16/creating-a-basic-webservice-in-rust.html)
- [Iron on uWSGI](http://i.shibe.ml/blog/?id=iron_on_uwsgi)
- [Deploying a Rust App to Google App Engine](http://blog.jecrooks.com/posts/rust-on-appengine.html)
- [async hyper](http://seanmonstar.com/post/141495445652/async-hyper)
- [Trying Rust for web services](https://blog.wearewizards.io/trying-rust-for-web-services)
- [Are we web yet?](http://www.arewewebyet.org)
- [Reimplementing ashurbanipal.web in Rust](http://maniagnosis.crsr.net/2015/07/reimplementing-ashurbanipalweb-in-rust.html)
- [A web app with Nickel: From first line to Heroku deployment](http://blog.thoughtram.io/rust/2015/07/29/a-web-app-with-nickel-from-first-line-to-heroku-deployment.html)
- [What features Iron does not have compared to a web server like nginx?](https://www.reddit.com/r/rust/comments/3t1mze/what_features_iron_does_not_have_compared_to_a/)
- [Build an API in Rust with JWT Authentication using Nickel.rs](https://auth0.com/blog/2015/11/30/build-an-api-in-rust-with-jwt-authentication-using-nickelrs/)
- [Selective Middleware for Iron](https://gregchapple.com/selective-middleware-for-iron/)
- [Rust for the Web - RESTful API in Rust, impressions](https://medium.com/@eugeniyoz/restful-api-in-rust-impressions-63250d611d15)
- [Rust for Node developers](https://github.com/Mercateo/rust-for-node-developers)

### Demos

- [exoskeleton](https://github.com/redo-studios/exoskeleton) - Iron
- [Example webapp using React + Webpack](https://github.com/cmsd2/rust-iron-react-webpack) - Iron
- [rustwebapp](https://github.com/superlogical/rustwebapp) - Iron and Postgres (r2d2)
- [webrust](https://github.com/Keats/webrust) - Iron and Postgres (r2d2)
- [httptest](https://github.com/brson/httptest) - Iron
- [nickel-todo-backend](https://github.com/Ryman/nickel-todo-backend/) - Nickel and Postgres (r2d2)
- [rust-playground](https://github.com/integer32llc/rust-playground) - Iron
- [rust-web-example](https://github.com/DavidBM/rust-webserver-example-with-iron-diesel-r2d2-serde) - Iron + Diesel (r2d2) + Serde
- [websocket chat](https://github.com/actix/examples/tree/master/websocket-chat) - Actix: Browser Websocket + tcp chat
- [diesel](https://github.com/actix/examples/tree/master/diesel) - Actix + Diesel
- [json](https://github.com/actix/examples/tree/master/json) - Actix + serde\_json or json\_rust

### Real-world web projects using Rust

- [paste.rs](https://paste.rs/) - Rocket
- [realworld-rust-rocket](https://github.com/TatriX/realworld-rust-rocket) - Rocket + Diesel, medium.com clone example
- [Portier](https://portier.github.io/) - Iron and Redis
- [yaus](https://github.com/gsquire/yaus) - Iron and SQLite
- [racerd](https://github.com/jwilm/racerd) - Iron
- [rust-passivetotal](https://github.com/passivetotal/rust_api) - Hyper
- [mars](https://github.com/Ticki/mars) - Hyper
- [openfairdb](https://github.com/flosse/openfairdb) - Rocket and Neo4j (r2d2)
- [ruma](https://github.com/ruma/ruma) - Iron and Posgres (diesel + r2d2)
- [html2pdf](https://github.com/rap2hpoutre/htmltopdf) - Iron
- [Hagrid](https://gitlab.com/hagrid-keyserver/hagrid) (keys.openpgp.org) - Rocket

### JS & [asm.js](http://asmjs.org/) & [WASM](http://webassembly.org/)

- [hellorust.com](https://www.hellorust.com) - a website with news, resources and demos

#### Examples

- [rust-webapp-template](https://github.com/huytd/rust-webapp-template) - Template project for Rust web app using *stdweb*
- [rust-todomvc](http://github.com/tcr/rust-todomvc) - an example application build with webplatform
- [wasm-experiments](https://github.com/killercup/wasm-experiments) - experiments with `wasm32-unknown-unknown`

#### Benchmark

- [TechEmpower Web Framework Benchmarks](https://www.techempower.com/benchmarks/#section=data-r15&hw=ph&test=plaintext)
- [web-frameworks](https://github.com/the-benchmarker/web-frameworks) - Measuring response times (routing times) for each framework (middleware). Each framework has to have two features; routing and parsing path parameters.
