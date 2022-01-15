# Rust web framework comparison

A comparison of some web frameworks written in [Rust](https://www.rust-lang.org).

This overview only contains frameworks that work on stable Rust.

## Table of Contents

- [Rust web framework comparison](#rust-web-framework-comparison)
  - [Table of Contents](#table-of-contents)
  - [Frontend frameworks (WASM)](#frontend-frameworks-wasm)
    - [Outdated frontend frameworks](#outdated-frontend-frameworks)
  - [Server frameworks](#server-frameworks)
    - [High-Level Server Frameworks](#high-level-server-frameworks)
    - [Low-Level Frameworks](#low-level-frameworks)
    - [Outdated server frameworks](#outdated-server-frameworks)
  - [Client frameworks](#client-frameworks)
    - [Outdated client frameworks](#outdated-client-frameworks)
  - [Supplemental libraries](#supplemental-libraries)
    - [Templating](#templating)
    - [Websocket Libraries](#websocket-libraries)
  - [Resources](#resources)
    - [Blog posts](#blog-posts)
      - [2018](#2018)
      - [Until 2017](#until-2017)
    - [Demos](#demos)
    - [Real-world web projects using Rust](#real-world-web-projects-using-rust)
    - [JS & asm.js & [WASM](http://webassembly.org/)](#js--asmjs--wasm)
      - [Examples](#examples)
      - [Benchmark](#benchmark)

## Frontend frameworks (WASM)

Since [WASM](http://webassembly.org/) support is available in most browsers we can use Rust to build web applications :)

|              Name | [yew](https://yew.rs)                                                                       | [seed](http://seed-rs.org/)                                                                        | sauron                                                                                             | mogwai                                                                                          | dominator                                                                                                 | [sycamore](https://sycamore-rs.netlify.app)                                                              |
| ----------------: | ------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
|    **Repository** | [![Repo](https://img.shields.io/badge/GitHub-git-blue)](https://github.com/yewstack/yew)    | [![Repo](https://img.shields.io/badge/GitHub-git-blue)](https://github.com/seed-rs/seed)           | [![Repo](https://img.shields.io/badge/GitHub-git-blue)](https://github.com/ivanceras/sauron)       | [![Repo](https://img.shields.io/badge/GitHub-git-blue)](https://github.com/schell/mogwai)       | [![Repo](https://img.shields.io/badge/GitHub-git-blue)](https://github.com/Pauan/rust-dominator)          | [![Repo](https://img.shields.io/badge/GitHub-git-blue)](https://github.com/sycamore-rs/sycamore)         |
| **Documentation** | [![Docs](https://img.shields.io/badge/docs.rs-yew-green)](https://docs.rs/yew/)             | [![Docs](https://img.shields.io/badge/docs.rs-seed-green)](https://docs.rs/seed/)                  | [![Docs](https://img.shields.io/badge/docs.rs-sauron-green)](https://docs.rs/sauron/)              | [![Docs](https://img.shields.io/badge/docs.rs-mogwai-green)](https://docs.rs/mogwai/)           | [![Docs](https://img.shields.io/badge/docs.rs-dominator-green)](https://docs.rs/dominator/)               | [![Docs](https://img.shields.io/badge/docs.rs-sycamore-green)](https://docs.rs/sycamore/)                |
|       **License** | ![Yew license](https://img.shields.io/crates/l/yew.svg?label=%20)                           | ![Seed license](https://img.shields.io/crates/l/seed.svg?label=%20)                                | ![sauron license](https://img.shields.io/crates/l/sauron.svg?label=%20)                            | ![Mogwai license](https://img.shields.io/crates/l/mogwai.svg?label=%20)                         | ![Dominator license](https://img.shields.io/crates/l/dominator.svg?label=%20)                             | ![Sycamore license](https://img.shields.io/crates/l/sycamore.svg?label=%20)                              |
|       **Version** | ![Yew version](https://img.shields.io/crates/v/yew.svg?label=%20)                           | ![Seed version](https://img.shields.io/crates/v/seed.svg?label=%20)                                | ![sauron version](https://img.shields.io/crates/v/sauron.svg?label=%20)                            | ![Mogwai version](https://img.shields.io/crates/v/mogwai.svg?label=%20)                         | ![Dominator version](https://img.shields.io/crates/v/dominator.svg?label=%20)                             | ![Sycamore version](https://img.shields.io/crates/v/sycamore.svg?label=%20)                              |
|  **Github Stars** | ![Yew stars](https://img.shields.io/github/stars/yewstack/yew.svg?label=%20)                | ![Seed stars](https://img.shields.io/github/stars/David-OConnor/seed.svg?label=%20)                | ![sauron stars](https://img.shields.io/github/stars/ivanceras/sauron.svg?label=%20)                | ![Mogwai stars](https://img.shields.io/github/stars/schell/mogwai.svg?label=%20)                | ![Dominator stars](https://img.shields.io/github/stars/Pauan/rust-dominator.svg?label=%20)                | ![Sycamore stars](https://img.shields.io/github/stars/sycamore-rs/sycamore.svg?label=%20)                |
|  **Contributors** | ![Yew contributors](https://img.shields.io/github/contributors/yewstack/yew.svg?label=%20)  | ![Seed contributors](https://img.shields.io/github/contributors/David-OConnor/seed.svg?label=%20)  | ![sauron contributors](https://img.shields.io/github/contributors/ivanceras/sauron.svg?label=%20)  | ![Mogwai contributors](https://img.shields.io/github/contributors/schell/mogwai.svg?label=%20)  | ![Dominator contributors](https://img.shields.io/github/contributors/Pauan/rust-dominator.svg?label=%20)  | ![Sycamore contributors](https://img.shields.io/github/contributors/sycamore-rs/sycamore.svg?label=%20)  |
|      **Activity** | ![Yew activity](https://img.shields.io/github/commit-activity/y/yewstack/yew.svg?label=%20) | ![Seed activity](https://img.shields.io/github/commit-activity/y/David-OConnor/seed.svg?label=%20) | ![sauron activity](https://img.shields.io/github/commit-activity/y/ivanceras/sauron.svg?label=%20) | ![Mogwai activity](https://img.shields.io/github/commit-activity/y/schell/mogwai.svg?label=%20) | ![Dominator activity](https://img.shields.io/github/commit-activity/y/Pauan/rust-dominator.svg?label=%20) | ![Sycamore activity](https://img.shields.io/github/commit-activity/y/sycamore-rs/sycamore.svg?label=%20) |
|   **Virtual DOM** | yes                                                                                         | yes                                                                                                | yes                                                                                                | **no**                                                                                          | **no**                                                                                                    | **no**                                                                                                   |
|           **SSR** | no                                                                                          | no                                                                                                 | **yes**                                                                                            | **yes**                                                                                         | no                                                                                                        | **yes**                                                                                                  |

### Outdated frontend frameworks

- [dodrio](https://github.com/fitzgen/dodrio)
- [draco](https://github.com/utkarshkukreti/draco)
- [dumle](https://github.com/axelf4/dumle)
- [mika](https://gitlab.com/limira-rs/mika)
- [percy](https://github.com/chinedufn/percy)
- [ruukh](https://github.com/csharad/ruukh)
- [smithy](https://github.com/rbalicki2/smithy)
- [squark](https://github.com/rail44/squark)
- [stdweb](https://github.com/koute/stdweb)
- [willow](https://github.com/sindreij/willow)

## Server frameworks

### High-Level Server Frameworks

This overview only contains frameworks that support async execution.

| Name               | rocket                                                                                                 | warp                                                                                             | actix-web                                                                                            | gotham                                                                                             | Thruster                                                                                           | tide                                                                                         | salvo                                                                                           | trillium                                                                                                 | axum                                                                                          |
| ------------------ | ------------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------- |
| **License**        | ![Rocket license](https://img.shields.io/crates/l/rocket.svg?label=%20)                                | ![warp license](https://img.shields.io/crates/l/warp.svg?label=%20)                              | ![Actix-web license](https://img.shields.io/crates/l/actix-web.svg?label=%20)                        | ![Gotham license](https://img.shields.io/crates/l/gotham.svg?label=%20)                            | ![Thruster license](https://img.shields.io/crates/l/thruster.svg?label=%20)                        | ![Tide license](https://img.shields.io/crates/l/tide.svg?label=%20)                          | ![Salvo license](https://img.shields.io/crates/l/salvo.svg?label=%20)                           | ![Trillium license](https://img.shields.io/crates/l/trillium.svg?label=%20)                              | ![Axum license](https://img.shields.io/crates/l/axum.svg?label=%20)                           |
| **Version**        | ![Rocket version](https://img.shields.io/crates/v/rocket.svg?label=%20)                                | ![warp version](https://img.shields.io/crates/v/warp.svg?label=%20)                              | ![Actix-web version](https://img.shields.io/crates/v/actix-web.svg?label=%20)                        | ![Gotham version](https://img.shields.io/crates/v/gotham.svg?label=%20)                            | ![Thruster version](https://img.shields.io/crates/v/thruster.svg?label=%20)                        | ![Tide version](https://img.shields.io/crates/v/tide.svg?label=%20)                          | ![Salvo version](https://img.shields.io/crates/v/salvo.svg?label=%20)                           | ![Trillium version](https://img.shields.io/crates/v/trillium.svg?label=%20)                              | ![Axum version](https://img.shields.io/crates/v/axum.svg?label=%20)                           |
| **Github Stars**   | ![Rocket stars](https://img.shields.io/github/stars/SergioBenitez/Rocket.svg?label=%20)                | ![warp stars](https://img.shields.io/github/stars/seanmonstar/warp.svg?label=%20)                | ![Actix-web stars](https://img.shields.io/github/stars/actix/actix-web.svg?label=%20)                | ![Gotham stars](https://img.shields.io/github/stars/gotham-rs/gotham.svg?label=%20)                | ![Thruster stars](https://img.shields.io/github/stars/trezm/Thruster.svg?label=%20)                | ![Tide stars](https://img.shields.io/github/stars/http-rs/tide.svg?label=%20)                | ![Salvo stars](https://img.shields.io/github/stars/salvo-rs/salvo.svg?label=%20)                | ![Trillium stars](https://img.shields.io/github/stars/trillium-rs/trillium.svg?label=%20)                | ![Axum stars](https://img.shields.io/github/stars/tokio-rs/axum.svg?label=%20)                |
| **Contributors**   | ![Rocket contributors](https://img.shields.io/github/contributors/SergioBenitez/Rocket.svg?label=%20)  | ![warp contributors](https://img.shields.io/github/contributors/seanmonstar/warp.svg?label=%20)  | ![Actix-web contributors](https://img.shields.io/github/contributors/actix/actix-web.svg?label=%20)  | ![Gotham contributors](https://img.shields.io/github/contributors/gotham-rs/gotham.svg?label=%20)  | ![Thruster contributors](https://img.shields.io/github/contributors/trezm/Thruster.svg?label=%20)  | ![Tide contributors](https://img.shields.io/github/contributors/http-rs/tide.svg?label=%20)  | ![Salvo contributors](https://img.shields.io/github/contributors/salvo-rs/salvo.svg?label=%20)  | ![Trillium contributors](https://img.shields.io/github/contributors/trillium-rs/trillium.svg?label=%20)  | ![Axum contributors](https://img.shields.io/github/contributors/tokio-rs/axum.svg?label=%20)  |
| **Activity**       | ![Rocket activity](https://img.shields.io/github/commit-activity/y/SergioBenitez/Rocket.svg?label=%20) | ![warp activity](https://img.shields.io/github/commit-activity/y/seanmonstar/warp.svg?label=%20) | ![Actix-web activity](https://img.shields.io/github/commit-activity/y/actix/actix-web.svg?label=%20) | ![Gotham activity](https://img.shields.io/github/commit-activity/y/gotham-rs/gotham.svg?label=%20) | ![Thruster activity](https://img.shields.io/github/commit-activity/y/trezm/Thruster.svg?label=%20) | ![Tide activity](https://img.shields.io/github/commit-activity/y/http-rs/tide.svg?label=%20) | ![Salvo activity](https://img.shields.io/github/commit-activity/y/salvo-rs/salvo.svg?label=%20) | ![Trillium activity](https://img.shields.io/github/commit-activity/y/trillium-rs/trillium.svg?label=%20) | ![Axum activity](https://img.shields.io/github/commit-activity/y/tokio-rs/axum.svg?label=%20) |
| **Base framework** | hyper                                                                                                  | hyper                                                                                            | tokio                                                                                                | hyper                                                                                              | tokio (or hyper)                                                                                   | async-std                                                                                    | hyper                                                                                           |                                                                                                          | hyper                                                                                         |
| **HTTPS support**  | yes                                                                                                    | yes                                                                                              | yes                                                                                                  | yes                                                                                                | yes                                                                                                |                                                                                              | yes                                                                                             | yes                                                                                                      |                                                                                               |
| **HTTP/2 support** | yes                                                                                                    | yes                                                                                              | yes                                                                                                  | no                                                                                                 | yes                                                                                                |                                                                                              | yes                                                                                             | no                                                                                                       |                                                                                               |

- **actix-web** ([homepage](https://actix.rs/)            / [repository](https://github.com/actix/actix-web)       / [documentation](https://actix.github.io/actix-web/actix_web/) / [user guide](https://actix.rs/docs/))
- **axum** (            / [repository](https://github.com/tokio-rs/axum)       / [documentation](https://docs.rs/axum) / [examples](https://github.com/tokio-rs/axum/tree/main/examples) )
- **gotham**    ([homepage](http://gotham.rs/)            / [repository](https://github.com/gotham-rs/gotham/)     / [documentation](https://docs.rs/gotham/) / [examples](https://github.com/gotham-rs/gotham/tree/master/examples))
- **rocket**    ([homepage](https://rocket.rs/)           / [repository](https://github.com/SergioBenitez/rocket)  / [documentation](https://rocket.rs/guide/))
- **Thruster**  ( -                                       / [repository](https://github.com/trezm/Thruster)        / [documentation](https://docs.rs/thruster) / [examples](https://github.com/trezm/Thruster/tree/master/examples))
- **Tide**      ( -                                       / [repository](https://github.com/rustasync/tide)        / [documentation](https://docs.rs/tide) / [examples](https://github.com/rustasync/tide/tree/master/examples))
- **warp**      ( -                                       / [repository](https://github.com/seanmonstar/warp)      / [documentation](https://docs.rs/warp/) / [examples](https://github.com/seanmonstar/warp/tree/master/examples))
- **salvo**      ( -                                       / [repository](https://github.com/salvo-rs/salvo)      / [documentation](https://docs.rs/salvo/) / [examples](https://github.com/salvo-rs/salvo/tree/master/examples))
- **trillium** ([homepage](https://trillium.rs/)             / [repository](https://github.com/trillium-rs/trillium)       / [documentation](https://docs.trillium.rs/) / [user guide](https://trillium.rs/))

### Low-Level Frameworks

| Name               | [hyper](http://hyper.rs/)                                                                       | [tiny-http](https://github.com/frewsxcv/tiny-http)                                                       |
| ------------------ | ----------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------- |
| **License**        | ![Hyper license](https://img.shields.io/crates/l/hyper.svg?label=%20)                           | ![Tiny-http license](https://img.shields.io/crates/l/tiny-http.svg?label=%20)                            |
| **Version**        | ![Hyper version](https://img.shields.io/crates/v/hyper.svg?label=%20)                           | ![Tiny-http version](https://img.shields.io/crates/v/tiny-http.svg?label=%20)                            |
| **Github Stars**   | ![Hyper stars](https://img.shields.io/github/stars/hyperium/hyper.svg?label=%20)                | ![Tiny-http stars](https://img.shields.io/github/stars/tiny-http/tiny-http.svg?label=%20)                |
| **Contributors**   | ![Hyper contributors](https://img.shields.io/github/contributors/hyperium/hyper.svg?label=%20)  | ![Tiny-http contributors](https://img.shields.io/github/contributors/tiny-http/tiny-http.svg?label=%20)  |
| **Activity**       | ![Hyper activity](https://img.shields.io/github/commit-activity/y/hyperium/hyper.svg?label=%20) | ![Tiny-http activity](https://img.shields.io/github/commit-activity/y/tiny-http/tiny-http.svg?label=%20) |
| **Server**         | yes                                                                                             | yes                                                                                                      |
| **Client**         | yes                                                                                             | no                                                                                                       |
| **HTTPS support**  | yes                                                                                             | yes                                                                                                      |
| **HTTP/2 support** | yes (h2)                                                                                        | no                                                                                                       |
| **Async**          | yes                                                                                             | no                                                                                                       |

If you need a more low level control you can choose between these libraries:

- **hyper**     ([homepage](http://hyper.rs/) / [repository](https://github.com/hyperium/hyper)       / [documentation](https://docs.rs/hyper/))
- **tiny-http** ( -                           / [repository](https://github.com/tiny-http/tiny-http))  / [documentation](https://docs.rs/tiny_http/))

### Outdated server frameworks

- [civet](https://github.com/wycats/rust-civet)
- [conduit](https://github.com/conduit-rust/conduit)
- [cargonaouts](https://github.com/cargonauts-rs/cargonauts)
- [hydrogen](https://github.com/nathansizemore/hydrogen)
- [iron](https://github.com/iron/iron)
- [kinglet](https://github.com/pyfisch/kinglet)
- [nickel](https://github.com/nickel-org/nickel.rs/)
- [rouille](https://github.com/tomaka/rouille)
- [rustless](https://github.com/rustless/rustless)
- [rustful](https://github.com/Ogeon/rustful)
- [shio](https://github.com/mehcode/shio-rs)
- [sappers](https://github.com/sappworks/sapper)
- [solicit](https://github.com/mlalic/solicit)
- [tower-web](https://github.com/carllerche/tower-web)
- [tk-http](https://github.com/swindon-rs/tk-http)

## Client frameworks

To build web clients with Rust, you can choose between these libraries:

- **actix-web**  ([homepage](https://actix.rs/) / [repository](https://github.com/actix/actix-web)         / [api docs](https://actix.github.io/actix-web/actix_web/client/index.html))
- **reqwest**    (-                             / [repository](https://github.com/seanmonstar/reqwest)     / [documentation](https://docs.rs/reqwest))
- **hyper**      ([homepage](http://hyper.rs/)  / [repository](https://github.com/hyperium/hyper)          / [documentation](http://hyper.rs/hyper/hyper/))
- **jsonrpc**    (-                             / [repository](https://github.com/apoelstra/rust-jsonrpc/) / [documentation](https://www.wpsoftware.net/rustdoc/jsonrpc/))

### Outdated client frameworks

- [ease](https://github.com/SimonPersson/ease)

## Supplemental libraries

### Templating

- **sailfish**       ([homepage](https://sailfish.netlify.app)          / [repository](https://github.com/Kogia-sima/sailfish/)     / [documentation](https://docs.rs/sailfish))
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

### Websocket Libraries

| Name               | websocket                                                                                                         | ws-rs                                                                                            | twist                                                                                             | tungstenite                                                                                                    | actix-web                                                                                            |
| ------------------ | ----------------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------ | ------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- |
| **License**        | ![Websocket license](https://img.shields.io/crates/l/websocket.svg?label=%20)                                     | ![Ws-rs license](https://img.shields.io/crates/l/ws.svg?label=%20)                               | ![Twist license](https://img.shields.io/crates/l/ws.svg?label=%20)                                | ![Tungstenite license](https://img.shields.io/crates/l/tungstenite.svg?label=%20)                              | ![Actix-web license](https://img.shields.io/crates/l/actix-web.svg?label=%20)                        |
| **Version**        | ![Websocket version](https://img.shields.io/crates/v/websocket.svg?label=%20)                                     | ![Ws-rs version](https://img.shields.io/crates/v/ws.svg?label=%20)                               | ![Twist version](https://img.shields.io/crates/v/ws.svg?label=%20)                                | ![Tungstenite version](https://img.shields.io/crates/v/tungstenite.svg?label=%20)                              | ![Actix-web version](https://img.shields.io/crates/v/actix-web.svg?label=%20)                        |
| **Github Stars**   | ![Websocket stars](https://img.shields.io/github/stars/websockets-rs/rust-websocket.svg?label=%20)                | ![Ws-rs stars](https://img.shields.io/github/stars/housleyjk/ws-rs.svg?label=%20)                | ![Twist stars](https://img.shields.io/github/stars/rustyhorde/twist.svg?label=%20)                | ![Tungstenite stars](https://img.shields.io/github/stars/snapview/tungstenite-rs.svg?label=%20)                | ![Actix-web stars](https://img.shields.io/github/stars/actix/actix-web.svg?label=%20)                |
| **Contributors**   | ![Websocket contributors](https://img.shields.io/github/contributors/websockets-rs/rust-websocket.svg?label=%20)  | ![Ws-rs contributors](https://img.shields.io/github/contributors/housleyjk/ws-rs.svg?label=%20)  | ![Twist contributors](https://img.shields.io/github/contributors/rustyhorde/twist.svg?label=%20)  | ![Tungstenite contributors](https://img.shields.io/github/contributors/snapview/tungstenite-rs.svg?label=%20)  | ![Actix-web contributors](https://img.shields.io/github/contributors/actix/actix-web.svg?label=%20)  |
| **Activity**       | ![Websocket activity](https://img.shields.io/github/commit-activity/y/websockets-rs/rust-websocket.svg?label=%20) | ![Ws-rs activity](https://img.shields.io/github/commit-activity/y/housleyjk/ws-rs.svg?label=%20) | ![Twist activity](https://img.shields.io/github/commit-activity/y/rustyhorde/twist.svg?label=%20) | ![Tungstenite activity](https://img.shields.io/github/commit-activity/y/snapview/tungstenite-rs.svg?label=%20) | ![Actix-web activity](https://img.shields.io/github/commit-activity/y/actix/actix-web.svg?label=%20) |
| **Server**         | yes                                                                                                               | yes                                                                                              | yes                                                                                               | yes                                                                                                            | yes                                                                                                  |
| **Client**         | yes                                                                                                               | yes                                                                                              | yes                                                                                               | yes                                                                                                            | yes                                                                                                  |
| **Base framework** | - / tokio                                                                                                         | mio                                                                                              | tokio                                                                                             | - / tokio                                                                                                      | tokio                                                                                                |
| **Async**          | no / yes                                                                                                          | yes                                                                                              | yes                                                                                               | no / yes                                                                                                       | yes                                                                                                  |

- **websocket**   ([homepage](https://websockets-rs.github.io/rust-websocket/)   / [repository](https://github.com/websockets-rs/rust-websocket)  / [documentation](https://websockets-rs.github.io/rust-websocket/doc/websocket/))
- **ws-rs**       ([homepage](https://ws-rs.org)                           / [repository](https://github.com/housleyjk/ws-rs)          / [documentation](https://ws-rs.org/docs))
- **tungstenite** ( -                                                      / [repository](https://github.com/snapview/tungstenite-rs)  / [documentation](https://docs.rs/crate/tungstenite/))
- **tokio-tungstenite** ( -                                                      / [repository](https://github.com/snapview/tokio-tungstenite)  / [documentation](https://docs.rs/tokio-tungstenite))
- **actix-web**   ([homepage](https://actix.rs/)                           / [repository](https://github.com/actix/actix-web)          / [documentation](https://actix.github.io/actix-web/actix_web/))

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
- [rustwebapp](https://github.com/kordless/rustwebapp) - Iron and Postgres (r2d2)
- [webrust](https://github.com/Keats/webrust) - Iron and Postgres (r2d2)
- [httptest](https://github.com/brson/httptest) - Iron
- [nickel-todo-backend](https://github.com/Ryman/nickel-todo-backend/) - Nickel and Postgres (r2d2)
- [rust-playground](https://github.com/integer32llc/rust-playground) - Iron
- [rust-web-example](https://github.com/DavidBM/rust-webserver-example-with-iron-diesel-r2d2-serde) - Iron + Diesel (r2d2) + Serde
- [websocket chat](https://github.com/actix/examples/tree/master/websocket-chat) - Actix: Browser Websocket + tcp chat
- [diesel](https://github.com/actix/examples/tree/master/diesel) - Actix + Diesel
- [json](https://github.com/actix/examples/tree/master/json) - Actix + serde\_json or json\_rust

### Real-world web projects using Rust

- [Lemmy](https://github.com/LemmyNet/lemmy) - Actix + Diesel
- [paste.rs](https://paste.rs/) - Rocket
- [realworld-rust-rocket](https://github.com/TatriX/realworld-rust-rocket) - Rocket + Diesel, medium.com clone example
- [Portier](https://portier.github.io/) - Iron and Redis
- [yaus](https://github.com/gsquire/yaus) - Iron and SQLite
- [racerd](https://github.com/jwilm/racerd) - Iron
- [rust-passivetotal](https://github.com/passivetotal/rust_api) - Hyper
- [mars](https://github.com/Ticki/mars) - Hyper
- [openfairdb](https://github.com/flosse/openfairdb) - Rocket and SQLite (diesel + r2d2)
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
