# Rust web framework comparison

A comparison of some web frameworks written in [Rust](https://www.rust-lang.org).

## Server frameworks

There are several interesting frameworks to build web applications with Rust:

- **gotham**      ([homepage](http://gotham.rs/)                            / [repository](https://github.com/gotham-rs/gotham/)        / [documentation](https://book.gotham.rs/))
- **iron**        ([homepage](http://ironframework.io/)                     / [repository](https://github.com/iron/iron/)               / [documentation](http://ironframework.io/doc/iron/))
- **rocket**      ([homepage](https://rocket.rs/)                           / [repository](https://github.com/SergioBenitez/rocket)     / [documentation](https://rocket.rs/guide/))
- **nickel**      ([homepage](http://nickel-org.github.io/)                            / [repository](https://github.com/nickel-org/nickel.rs/)    / [documentation](http://nickel-org.github.io/nickel.rs))
- **rustful**     ( -                                                       / [repository](https://github.com/Ogeon/rustful)            / [documentation](http://ogeon.github.io/docs/rustful/master/rustful/))
- **rustless**    (-                                                        / [repository](https://github.com/rustless/rustless)        / - )
- **conduit**     ( -                                                       / [repository](https://github.com/conduit-rust/conduit)     / - )
- **rouille**     ( -                                                       / [repository](https://github.com/tomaka/rouille)           / [documentation](http://tomaka.github.io/rouille/rouille/index.html))
- **pencil**      ( -                                                       / [repository](https://github.com/fengsp/pencil)            / [documentation](http://fengsp.github.io/pencil/))
- **sappers**     ( -                                                       / [repository](https://github.com/sappworks/sapper)         / - )
- **boron**       ( -                                                       / [repository](https://github.com/troposphere/boron)        / - )
- **cargonaouts** ([homepage](https://cargonauts-rs.github.io/cargonauts/)  / [repository](https://github.com/cargonauts-rs/cargonauts) / - )
- **susanoo**     (-                                                        / [repository](https://github.com/ubnt-intrepid/susanoo/)   / - )
- **shio**        (-                                                        / [repository](https://github.com/mehcode/shio-rs)          / - )
- **actix-web**   (-                                                        / [repository](https://github.com/actix/actix-web)          / [documentation](https://actix.github.io/actix-web/actix_web/))

If you need a more low level control you can choose between these libraries:

- **hyper**     ([homepage](http://hyper.rs/) / [repository](https://github.com/hyperium/hyper)          / [documentation](http://hyper.rs/hyper))
- **tiny-http** ( -                           / [repository](https://github.com/frewsxcv/tiny-http))     / [documentation](http://frewsxcv.github.io/tiny-http/tiny_http/index.html))
- **solicit**   ( -                           / [repository](https://github.com/mlalic/solicit)          / [documentation](https://mlalic.github.io/solicit/solicit/index.html))
- **kinglet**   ( -                           / [repository](https://github.com/pyfisch/kinglet)         / - )
- **hydrogen**  ( -                           / [repository](https://github.com/nathansizemore/hydrogen) / [documentation](https://nathansizemore.github.io/hydrogen/hydrogen/index.html))
- **civet**     ( -                           / [repository](https://github.com/wycats/rust-civet)       / - )
- **tk-http**   ( -                           / [repository](https://github.com/swindon-rs/tk-http)      / - )
- **h2**        ( -                           / [repository](https://github.com/carllerche/h2)           / - )

## Client frameworks

To build web clients with Rust, you can choose between these libraries:

- **reqwest** (-                            / [repository](https://github.com/seanmonstar/reqwest)     / [documentation](https://docs.rs/reqwest))
- **hyper**   ([homepage](http://hyper.rs/) / [repository](https://github.com/hyperium/hyper)          / [documentation](http://hyper.rs/hyper/hyper/))
- **ease**    (-                            / [repository](https://github.com/SimonPersson/ease)       / [documentation](http://simonpersson.github.io/ease/))
- **jsonrpc** (-                            / [repository](https://github.com/apoelstra/rust-jsonrpc/) / [documentation](https://www.wpsoftware.net/rustdoc/jsonrpc/))

## Supplemental libraries

### Websocket

- **websocket**   ([homepage](http://cyderize.github.io/rust-websocket/)  / [repository](https://github.com/cyderize/rust-websocket)  / [documentation](http://cyderize.github.io/rust-websocket/doc/websocket/))
- **ws-rs**       ([homepage](https://ws-rs.org)                          / [repository](https://github.com/housleyjk/ws-rs)          / [documentation](https://ws-rs.org/docs))
- **tungstenite** (-                                                      / [repository](https://github.com/snapview/tungstenite-rs)  / [documentation](https://docs.rs/crate/tungstenite/))
- **tk-http**     ( -                                                     / [repository](https://github.com/swindon-rs/tk-http)       / - )
- **actix-web**   ( -                                                     / [repository](https://github.com/actix/actix-web)          / [documentation](https://actix.github.io/actix-web/actix_web/))

### Templating

- **tera**       (-                                                     / [repository](https://github.com/Keats/tera)               / - )
- **mustache**   (-                                                     / [repository](https://github.com/nickel-org/rust-mustache) / [documentation](http://nickel-org.github.io/rust-mustache))
- **liquid**     (-                                                     / [repository](https://github.com/cobalt-org/liquid-rust)   / - )
- **handlebars** (-                                                     / [repository](https://github.com/sunng87/handlebars-rust)  / - )
- **horrorshow** (-                                                     / [repository](https://github.com/Stebalien/horrorshow-rs)  / [documentation](https://stebalien.github.io/horrorshow-rs/horrorshow/))
- **maud**       ([homepage](https://lfairy.gitbooks.io/maud/content/)  / [repository](https://github.com/lfairy/maud)              / [documentation](https://lambda.xyz/maud/maud/))
- **askama**     (-                                                     / [repository](https://github.com/djc/askama)               / - )
 - **stpl**      (-                                                     / [repository](https://github.com/dpc/stpl)                 / - )

## Resources

### Blog posts

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
- [rust-web-example](https://github.com/DavidBM/rust-webserver-example-with-iron-diesel-r2d2-serde) - Iron + Diesel (r2d2) + Serde
- [websocket chat](https://github.com/actix/actix-web/tree/master/examples/websocket-chat) - Actix: Browser Websocket + tcp chat
- [diesel](https://github.com/actix/actix-web/tree/master/examples/diesel) - Actix + Diesel
- [json](https://github.com/actix/actix-web/tree/master/examples/json) - Actix + serde\_json or json\_rust

### Real-world web projects using Rust

- [paste.rs](https://paste.rs/) - Rocket
- [Portier](https://portier.github.io/) - Iron and Redis
- [yaus](https://github.com/gsquire/yaus) - Iron and SQLite
- [racerd](https://github.com/jwilm/racerd) - Iron
- [rust-passivetotal](https://github.com/passivetotal/rust_api) - Hyper
- [mars](https://github.com/Ticki/mars) - Hyper
- [openfairdb](https://github.com/flosse/openfairdb) - Rocket and Neo4j (r2d2)
- [ruma](https://github.com/ruma/ruma) - Iron and Posgres (diesel + r2d2)
- [html2pdf](https://github.com/rap2hpoutre/htmltopdf) - Iron
- [Kaizen 改善日本語 Japanese](https://kaizenjapanese.com) - Hyper


### JS & [asm.js](http://asmjs.org/) & [WASM](http://webassembly.org/)

- [stdweb](https://github.com/koute/stdweb) - A standard library for the client-side Web
- [webplatform](https://github.com/tcr/rust-webplatform) - a library for use with emscripten to access the DOM.
- [hellorust.com](https://www.hellorust.com) - a website with news, resources and demos

#### Examples

- [rust-webapp-template](https://github.com/huytd/rust-webapp-template) - Template project for Rust web app using *stdweb*
- [rust-todomvc](http://github.com/tcr/rust-todomvc) - an example application build with webplatform
- [wasm-experiments](https://github.com/killercup/wasm-experiments) - experiments with `wasm32-unknown-unknown`

#### Benchmark

- [benchmarks](https://github.com/fafhrd91/benchmarks) - Rust web frameworks benchmarks
- [which_is_the_fastest](https://github.com/tbrand/which_is_the_fastest) - Measuring response times (routing times) for each framework (middleware). Each framework has to have two features; routing and parsing path parameters.


## Comparison

### High-Level Frameworks

|        Name        | iron  | gotham           |      rocket      | nickel | rustful | rustless | conduit |  rouille   | ease  | jsonrpc |    pencil    | sappers | boron |     susanoo      |     shio         |    actix-web     |
| ------------------ | ----- | ---------------- | ---------------- | ------ | ------- | -------- | ------- | ---------- | ----- | ------- | ------------ | ------- | ----- | ---------------- | ---------------- | ---------------- |
| **License**        | MIT   | MIT / Apache 2.0 | MIT / Apache 2.0 | MIT    | MIT     | MIT      | MIT     | Apache 2.0 | MIT   | CC0-1.0 | BSD-3-Clause | MIT     | MIT   | MIT / Apache 2.0 | MIT / Apache 2.0 | MIT / Apache 2.0       |
| **Github Stars**   | 4.4k  | 0.3k             | 2.7k             | 1.9k   | 0.8k    | 0.3k     | 0.1k    | 0.1k       | 0.1k  | 0k      | 0.8k         | 0.4k    | 0k    | 0k               | 0.1k             | 0.1k               |
| **Contributors**   | 67    | 5                | 28               | 49     | 11      | 11       | 5       | 4          | 2     | 2       | 4            | 1       | 2     | 1                | 3                | 1                |
| **Server**         | yes   | yes              | yes              | yes    | yes     | yes      | yes     | yes        | no    | no      | yes          | yes     | yes   | yes              | yes              | yes              |
| **Client**         | no    | no               | no               | no     | no      | no       | no      | no         | yes   | yes     | no           | no      | no    | no               | no               | no               |
| **Base framework** | hyper | hyper            | hyper            | hyper  | hyper   | iron     | civet   | tiny-http  | hyper | hyper   | hyper        | hyper   | hyper | hyper            | hyper            | tokio            |
| **HTTPS support**  | yes   | yes              |                  | no     | yes     | ?        | ?       | ?          | -     | -       | ?            | ?       | no    |                  |                  | yes              |
| **HTTP/2 support** | ?     | no               |                  | ?      | ?       | ?        | ?       | ?          | ?     | ?       | ?            | ?       | no    |                  |                  | yes              |
| **Async**          |       | yes              | no               |        |         |          |         |            |       |         |              |         |       | yes              | yes              | yes              |

### Low-Level Frameworks

|        Name        | civet  | hyper   | tiny-http  | solicit | kinglet | hydrogen | tk-http          | h2               |
| ------------------ | ------ | ------- | ---------- | ------- | ------- | -------- | ---------------- | ---------------- |
| **License**        | MIT    | MIT     | Apache 2.0 | MIT     | MIT     | MPL 2.0  | MIT / Apache 2.0 | MIT / Apache 2.0 |
| **Github Stars**   | 0k     | 2.7k    | 0.2k       | 0.2k    | 0.1k    | 0.4k     | 0.1k             | 0.1k             |
| **Contributors**   | 4      | 112     | 9          | 8       | 1       | 2        | 5                | 8                |
| **Server**         | yes    | yes     | yes        | yes     | yes     | yes      | yes              | yes              |
| **Client**         | no     | yes     | ?          | yes     | no      | no       | yes              | yes              |
| **HTTPS support**  |        | yes     | yes        | -       | -       | no       | yes              | no               |
| **HTTP/2 support** |        | solicit | ?          | yes     | -       | no       | no               | yes              |
| **Async**          |        | yes     |            |         |         |          | yes              | yes              |


### Middleware & Plugins

|           Name            |                    iron                    | gotham |                        nickel                         | conduit | rouille | rustful | rustless | pencil | sappers | actix-web |
| ------------------------- | ------------------------------------------ | ------ | ----------------------------------------------------- | ------- | ------- | ------- | -------- | ------ | ------- | --------- |
| **Static File Serving**   | [yes](https://github.com/iron/staticfile)  | no^    | yes                                                   | yes     | n/a     | ?       | ?        | yes    | ?       | [yes](https://actix.github.io/actix-web/guide/qs_12.html)       |
| **Mounting**              | [yes](https://github.com/iron/mount)       | yes    | yes                                                   | ?       | n/a     | ?       | ?        | ?      | ?       | [yes](https://actix.github.io/actix-web/guide/qs_3.html#application)         |
| **Logging**               | [yes](https://github.com/iron/logger)      | yes    | no                                                    | ?       | n/a     | ?       | ?        | yes    | ?       | [yes](https://actix.github.io/actix-web/guide/qs_10.html#logging)       |
| **JSON-Body-Parsing**     | [yes](https://github.com/iron/body-parser) | yes    | yes                                                   | ?       | n/a     | ?       | ?        | ?      | ?       | [yes](https://github.com/actix/actix-web/tree/master/examples/json)         |
| **Sessions**              | [yes](https://github.com/iron/session)     | yes    | ?                                                     | ?       | n/a     | ?       | ?        | ?      | ?       | [yes](https://actix.github.io/actix-web/guide/qs_10.html#user-sessions)         |
| **Cookies**               | [yes](https://github.com/iron/cookie)      | yes    | ?                                                     | ?       | n/a     | ?       | ?        | ?      | ?       | [yes](https://actix.github.io/actix-web/guide/qs_10.html#user-sessions)       |
| **PostgreSQL middleware** | ?                                          | no^    | [yes](https://github.com/nickel-org/nickel-postgres)  | ?       | n/a     | ?       | ?        | ?      | ?       | [yes](https://github.com/actix/actix-web/tree/master/examples/diesel)        |
| **SQLite middleware**     | ?                                          | no^    | [yes](https://github.com/flosse/nickel-sqlite)        | ?       | n/a     | ?       | ?        | ?      | ?       | [yes](https://github.com/actix/actix-web/tree/master/examples/diesel)         |
| **Redis middleware**      | ?                                          | no^    | [yes](https://github.com/matthewbentley/nickel-redis) | ?       | n/a     | ?       | ?        | ?      | ?       | [yes](https://github.com/actix/actix-redis)         |
| **MySQL middleware**      | ?                                          | no^    | [yes](https://github.com/zither/nickel-mysql)         | ?       | n/a     | ?       | ?        | ?      | ?       | [yes](https://github.com/actix/actix-web/tree/master/examples/diesel)        |

(^ Planned in current roadmap)

### Websocket Libraries

|        Name        | websocket | ws-rs | twist            | tungstenite      | tk-http          | actix-web
| ------------------ | --------- | ----- | ---------------- | ---------------- | ---------------- | ---------------- |
| **License**        | MIT       | MIT   | MIT / Apache 2.0 | MIT / Apache 2.0 | MIT / Apache 2.0 | MIT / Apache 2.0 |
| **Github Stars**   | 0.4k      | 0.5k  | 0k               | 0.1k             | 0.1k             | 0.1k             |
| **Contributors**   | 31        | 25    | 2                | 8                | 5                | 1                |
| **Server**         | yes       | yes   | yes              | yes              | yes              | yes              |
| **Client**         | yes       | yes   | yes              | yes              | yes              | no               |
| **Base framework** | - / tokio | mio   | tokio            | - / tokio        | tokio            | tokio            |
| **Async**          | no / yes  | yes   | yes              | no / yes         | yes              | yes              |


## Examples

To compile or run the examples use [Cargo](https://github.com/rust-lang/cargo).
First clone this repo

    git clone https://github.com/flosse/rust-web-framework-comparison
    cd rust-web-framework-comparison/

and change to the desired frameworkd directory (e.g. `cd iron/`) and type

    cargo run --example hello_world

Then visit `http://localhost:3000` to see the result.
