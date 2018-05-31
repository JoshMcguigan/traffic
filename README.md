# Traffic [![Build Status](https://travis-ci.org/JoshMcguigan/traffic.svg?branch=master)](https://travis-ci.org/JoshMcguigan/traffic)

Traffic is the easiest way to keep track of the traffic to your projects on Github. Traffic queries the Github API to build a traffic report for all of your projects.

![screenshot](https://user-images.githubusercontent.com/22216761/40780038-fc70cae8-648b-11e8-85b3-361a9a781461.png)

## Setup

Traffic requires Rust in order to build/install. If you need to install Rust, follow [these instructions](https://www.rust-lang.org/en-US/install.html). Once you have Rust installed, Traffic can be installed by running the command below:

```
cargo install traffic
```

The first time running Traffic, you will be asked to enter your Github credentials. If you have two factor authentication enabled on Github you'll need to setup a personal access token for Traffic, with the `read:user` and `repo` permissions.

To clear your Github credentials, use `traffic --logout`.

## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any
additional terms or conditions.
