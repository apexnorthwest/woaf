# Contribution and Development
All rules and guidelines in this document are expected to be followed by
contributors. Any pull requests that are found not to follow these rules will
be rejected. No worries though, we won't be mean about it. :) If your PR is
rejected due to a contribution guideline issue, we'll do our best to be clear
about why and what you can do to resolve it. We're all on the same team.

## Guidelines for Contributing
### Style and Linting
This project uses standard rust conventions. You should have an appropriate
amount of comments to make clear what your code does without going into too
much detail. Additionally, we expect the code to pass machete, cargo check,
cargo clippy, and be in line with cargo standard format. You can do the following
to format and check your code locally:

```shell
# Run these once to set up the toolchain
rustup update stable
rustup default stable
rustup component add clippy
cargo install cargo-machete

# Run the formatter and checks. This takes a while the first time
cargo fmt
cargo check
cargo clippy
cargo machete
```

### Vulnerability Scanning
Multiple vulnerability and code quality scanners are in use on this project.
Alerts from any of them should be considered mandatory prior to merging a pull
request against the project. Pull request CI pipelines will provide results. If
you are unable to view the alerts yourself, a maintainer will review them on your
behalf.

Should you discover a critical vulnerability, you are encouraged to report it via
<admin@apexnorthwest.com>. Should the maintainers fail to respond within 30 days,
you should open an issue on this repo. If an unpatched vulnerability is found in a
library or included binary, please open an issue right away with the appropriate details.

### Use of AI tools
Use of AI assisted coding tools is acceptable, but all code is expected to be
of high quality and comply with this standards document. All code should be
reviewed, tested, and understood by the contributor. You as the developer are
responsible for the code you submit, AI assisted or otherwise. You, as a developer,
must be able to explain and defend your code when asked.

Pull requests submitted by an automated agent or code written entirely by an
agent are very likely to be rejected. Autonomous AI agents are powerful, but
at this time they are still prone to subtle errors and misunderstanding of
the finer semantics of many applications. Bugs introduced by agents can be
difficult to debug, especially without anyone who knows how their code works.

This section is subject to change as these technologies develop rapidly. Our
stance is to remain conservative on what we permit. While these tools are
very valuable, and this project's maintainers do use some of them, there is
tangible value in having people who know and deeply understand code. As in all
things in life, balance is important.

## Development Guide
The basic workflow for contributing will look like the following:
1. Fork the repository and check out your own version of the project. You should also create a branch with a name the represents the work you're doing.
```sh
git clone https://github.com/<your username>/woaf.git
cd woaf
```
2. Set up your development machine (Linux):
```sh
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup update stable
rustup default stable
cargo check
cargo build
```
3. Do your development. This should be the fun part :)
4. Building the code with all assertions on (for Linux x64 in this example):
```sh
CARGO_BUILD_WARNINGS=deny RUSTFLAGS="-C target-feature=+crt-static" cargo build --target x86_64-unknown-linux-gnu
```
5. Run the test suite
```sh
cargo test
```
6. Run the fuzz tests
```sh
cargo install cargo-fuzz
cargo fuzz socket-offer
cargo fuzz socket-accept
```
7. Open a Pull Request  
  Your pull request should explain what problem you're trying to solve and link to an Issue if there's one that you expect
  to be fixed by the change. If it's a new feature, you must provide background on why the feature is needed and an example
  of both the problem and how to use the new feature.
