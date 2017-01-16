# Rust web framework comparison

A comparison of some web frameworks written in [Rust](https://www.rust-lang.org).

## Server frameworks

At the moment there are several interesting frameworks to build web applications
with Rust:

- **iron**     ([homepage](http://ironframework.io/) / [repository](https://github.com/iron/iron/)            / [documentation](http://ironframework.io/doc/iron/))
- **nickel**   ([homepage](http://nickel.rs/)        / [repository](https://github.com/nickel-org/nickel.rs/) / [documentation](http://docs.nickel.rs/nickel/))
- **rocket**   ([homepage](https://rocket.rs/)       / [repository](https://github.com/SergioBenitez/rocket)  / [documentation](https://rocket.rs/guide/) )
- **rustful**  ( -                                   / [repository](https://github.com/Ogeon/rustful)         / [documentation](http://ogeon.github.io/docs/rustful/master/rustful/))
- **rustless** ([homepage](http://rustless.org/)     / [repository](https://github.com/rustless/rustless)     / [documentation](http://rustless.org/rustless/doc/rustless/))
- **conduit**  ( -                                   / [repository](https://github.com/conduit-rust/conduit)  / - )
- **rouille**  ( -                                   / [repository](https://github.com/tomaka/rouille)        / [documentation](http://tomaka.github.io/rouille/rouille/index.html))
- **pencil**   ( -                                   / [repository](https://github.com/fengsp/pencil)         / [documentation](http://fengsp.github.io/pencil/))
- **sappers**  ( -                                   / [repository](https://github.com/sappworks/sapper)      / - )
- **boron**    ( -                                   / [repository](https://github.com/troposphere/boron)     / - )

If you need a more low level control you can choose between five libraries:

- **hyper**     ([homepage](http://hyper.rs/) / [repository](https://github.com/hyperium/hyper)          / [documentation](http://hyper.rs/hyper))
- **tiny-http** ( -                           / [repository](https://github.com/frewsxcv/tiny-http))     / [documentation](http://frewsxcv.github.io/tiny-http/tiny_http/index.html))
- **solicit**   ( -                           / [repository](https://github.com/mlalic/solicit)          / [documentation](https://mlalic.github.io/solicit/solicit/index.html))
- **kinglet**   ( -                           / [repository](https://github.com/pyfisch/kinglet)         / - )
- **hydrogen**  ( -                           / [repository](https://github.com/nathansizemore/hydrogen) / [documentation](https://nathansizemore.github.io/hydrogen/hydrogen/index.html))

## Client frameworks

To build web clients with Rust, you can choose between these libraries:

- **reqwest** (-                            / [repository](https://github.com/seanmonstar/reqwest)     / [documentation](https://docs.rs/reqwest))
- **hyper**   ([homepage](http://hyper.rs/) / [repository](https://github.com/hyperium/hyper)          / [documentation](http://hyper.rs/hyper/hyper/))
- **ease**    (-                            / [repository](https://github.com/SimonPersson/ease)       / [documentation](http://simonpersson.github.io/ease/))
- **jsonrpc** (-                            / [repository](https://github.com/apoelstra/rust-jsonrpc/) / [documentation](https://www.wpsoftware.net/rustdoc/jsonrpc/))

## Supplemental libraries

### Websocket

- **websocket** ([homepage](http://cyderize.github.io/rust-websocket/) / [repository](https://github.com/cyderize/rust-websocket)  / [documentation](http://cyderize.github.io/rust-websocket/doc/websocket/))
- **ws-rs**     ([homepage](https://ws-rs.org) /                                                [repository](https://github.com/housleyjk/ws-rs) / [documentation](https://ws-rs.org/docs))

### Templating

- **tera**       (-                                                     / [repository](https://github.com/Keats/tera)               / - )
- **mustache**   (-                                                     / [repository](https://github.com/nickel-org/rust-mustache) / [documentation](http://nickel-org.github.io/rust-mustache))
- **liquid**     (-                                                     / [repository](https://github.com/cobalt-org/liquid-rust)   / - )
- **handlebars** (-                                                     / [repository](https://github.com/sunng87/handlebars-rust)  / - )
- **horrorshow** (-                                                     / [repository](https://github.com/Stebalien/horrorshow-rs)  / [documentation](https://stebalien.github.io/horrorshow-rs/horrorshow/))
- **maud**       ([homepage](https://lfairy.gitbooks.io/maud/content/)  / [repository](https://github.com/lfairy/maud)              / [documentation](https://lambda.xyz/maud/maud/))

## Resources

### Blog posts

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
- [Introducing Pencil: A Microframework Inspired By Flask For Rust](https://fengsp.github.io/blog/2016/3/introducing-pencil/)
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

### Real-world web projects using Rust

- [Portier](https://portier.github.io/) - Iron and Redis
- [yaus](https://github.com/gsquire/yaus) - Iron and SQLite
- [racerd](https://github.com/jwilm/racerd) - Iron
- [rust-passivetotal](https://github.com/passivetotal/rust_api) - Hyper
- [mars](https://github.com/Ticki/mars) - Hyper
- [openfairdb](https://github.com/flosse/openfairdb) - Nickel and Neo4j (r2d2)
- [ruma](https://github.com/ruma/ruma) - Iron and Posgres (diesel + r2d2)
- [html2pdf](https://github.com/rap2hpoutre/htmltopdf) - Iron

### JS & [asm.js](http://asmjs.org/) & [WASM](http://webassembly.org/)

- [Compiling to the web with Rust and emscripten](https://users.rust-lang.org/t/compiling-to-the-web-with-rust-and-emscripten/7627)
- [webplatform](https://github.com/tcr/rust-webplatform) - a library for use with emscripten to access the DOM.
- [rust-todomvc](http://github.com/tcr/rust-todomvc) - an example application build with webplatform

## Comparison

### General

|                      Name | iron   | nickel  | rocket            | rustful   | rustless  | conduit | rouille    | hyper   | tiny-http  | solicit | ease  | jsonrpc | websocket |  ws-rs     | kinglet   |  pencil       |  sappers      | hydrogen     |  boron        |
| ------------------------- | ------ | ------- | ----------------  | --------- | --------- | ------- | ---------- | ------- | ---------- | ------- | ----- | ------- | --------- |  --------- | --------- |  ------------ |  ------------ | ------------ |  ------------ |
|               **License** | MIT    | MIT     | MIT / Apache 2.0  | MIT       | MIT       | MIT     | Apache 2.0 | MIT     | Apache 2.0 | MIT     | MIT   | CC0-1.0 | MIT       |  MIT       | MIT       |  BSD-3-Clause |  MIT          | MPL 2.0      |  MIT          |
|          **Github Stars** | 3.5k   | 1.7k    | 1.3k              | 0.8k      | 0.3k      | 0.1k    | 0.1k       | 2.0k    | 0.2k       | 0.2k    | 0.1k  | 0k      | 0.2k      |  0.1k      | 0.1k      |  0.8k         |  0.4k         | 0.4k         |  0k           |
|          **Contributors** | 67     | 49      | 14                | 11        | 11        | 5       | 4          | 112     | 9          | 8       | 2     | 2       | 16        |  2         | 1         |  4            |  1            | 2            |  2            |
|                **Server** | yes    | yes     | yes               | yes       | yes       | yes     | yes        | yes     | yes        | yes     | no    | no      | yes       |  yes       | yes       |  yes          |  yes          | yes          |  yes          |
|                **Client** | no     | no      | no                | no        | no        | no      | no         | yes     | ?          | yes     | yes   | yes     | yes       |  yes       | no        |  no           |  no           | no           |  no           |
|        **Base framework** | hyper  | hyper   |                   | hyper     | iron      | civet   | tiny-http  | yes     | yes        | yes     | hyper | hyper   | no        |  no        | yes       |  hyper        |  hyper        | yes          |  hyper        |
|         **HTTPS support** | yes    | no      |                   | yes       | ?         | ?       | ?          | yes     | yes        | -       | -     | -       | -         |  yes       | -         |  ?            |  ?            | no           |  no           |
|        **HTTP/2 support** | ?      | ?       |                   | ?         | ?         | ?       | ?          | solicit | ?          | yes     | ?     | ?       | -         |  -         | -         |  ?            |  ?            | no           |  no           |

### Middleware & Plugins

|                      Name | iron                                        | nickel                                                | conduit | rouille | rustful   | rustless  | pencil    | sappers   |
| ------------------------- | ------------------------------------------- | ----------------------------------------------------- | ------- | ------- | --------- | --------- | --------- | --------- |
|   **Static File Serving** | [yes](https://github.com/iron/staticfile)   | yes                                                   | yes     | n/a     | ?         | ?         | yes       | ?         |
|              **Mounting** | [yes](https://github.com/iron/mount)        | yes                                                   | ?       | n/a     | ?         | ?         | ?         | ?         |
|               **Logging** | [yes](https://github.com/iron/logger)       | no                                                    | ?       | n/a     | ?         | ?         | yes       | ?         |
|     **JSON-Body-Parsing** | [yes](https://github.com/iron/body-parser)  | yes                                                   | ?       | n/a     | ?         | ?         | ?         | ?         |
|              **Sessions** | [yes](https://github.com/iron/session)      | ?                                                     | ?       | n/a     | ?         | ?         | ?         | ?         |
|               **Cookies** | [yes](https://github.com/iron/cookie)       | ?                                                     | ?       | n/a     | ?         | ?         | ?         | ?         |
| **PostgreSQL middleware** | ?                                           | [yes](https://github.com/nickel-org/nickel-postgres)  | ?       | n/a     | ?         | ?         | ?         | ?         |
|     **SQLite middleware** | ?                                           | [yes](https://github.com/flosse/nickel-sqlite)        | ?       | n/a     | ?         | ?         | ?         | ?         |
|      **Redis middleware** | ?                                           | [yes](https://github.com/matthewbentley/nickel-redis) | ?       | n/a     | ?         | ?         | ?         | ?         |
|      **MySQL middleware** | ?                                           | [yes](https://github.com/zither/nickel-mysql)         | ?       | n/a     | ?         | ?         | ?         | ?         |

## Examples

To compile or run the examples use [Cargo](https://github.com/rust-lang/cargo).
First clone this repo

    git clone https://github.com/flosse/rust-web-framework-comparison
    cd rust-web-framework-comparison/

and change to the desired frameworkd directory (e.g. `cd iron/`) and type

    cargo run --example hello_world

Then visit `http://localhost:3000` to see the result.
