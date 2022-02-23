# Contributing

A big welcome and thank you for considering contributing to **systeroid**! It is people like you that make it a reality for users in the open source community.

Reading and following these guidelines will help us make the contribution process easy and effective for everyone involved. It also communicates that you agree to respect the time of the developers managing and developing this project. In return, we will reciprocate that respect by addressing your issue, assessing changes, and helping you finalize your pull requests.

## Quicklinks

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
  - [Issues](#issues)
  - [Pull Requests](#pull-requests)
- [License](#license)

## Code of Conduct

We take our open source community seriously and hold ourselves and other contributors to high standards of communication. By participating and contributing to this project, you agree to uphold our [Code of Conduct](./CODE_OF_CONDUCT.md).

## Getting Started

Contributions are made to this repo via Issues and Pull Requests (PRs). A few general guidelines that cover both:

- First, discuss the change you wish to make via creating an [issue](https://github.com/orhun/systeroid/issues/new/choose), [email](mailto:orhunparmaksiz@gmail.com), or any other method with the owners of this repository before making a change.
- Search for existing issues and PRs before creating your own.
- We work hard to make sure issues are handled in a timely manner but, depending on the impact, it could take a while to investigate the root cause. A friendly ping in the comment thread to the submitter or a contributor can help draw attention if your issue is blocking.

### Issues

Issues should be used to report problems with the project, request a new feature, or discuss potential changes before a PR is created. When you create a new issue, a template will be loaded that will guide you through collecting and providing the information we need to investigate.

If you find an issue that addresses the problem you're having, please add your own reproduction information to the existing issue rather than creating a new one. Adding a [reaction](https://github.blog/2016-03-10-add-reactions-to-pull-requests-issues-and-comments/) can also help be indicating to our maintainers that a particular problem is affecting more than just the reporter.

### Pull Requests

PRs are always welcome and can be a quick way to get your fix or improvement slated for the next release. In general, PRs should:

- Only fix/add the functionality in question **or** address wide-spread whitespace/style issues, not both.
- Add unit or integration tests for fixed or changed functionality (if a test suite already exists).
- Address a single concern in the least number of changed lines as possible.
- Include documentation.
- Be accompanied by a complete Pull Request template (loaded automatically when a PR is created).

For changes that address core functionality or would require breaking changes (e.g. a major release), it's best to open an issue to discuss your proposal first. This is not required but can save time creating and reviewing changes.

In general, we follow the "[fork-and-pull](https://github.com/susam/gitpr)" Git workflow:

1. Fork the repository to your own GitHub account.

2. Clone the project to your local environment.

```sh
git clone https://github.com/<user>/systeroid && cd systeroid/
```

3. Create a branch locally with a succinct but descriptive name.

```sh
git checkout -b <branch_name>
```

4. Make sure you have everything installed in the [requirements](./README.md#requirements) section. If so, build the project.

```sh
cargo build
```

5. Start committing changes to the branch.

6. Add your tests or update the existing tests according to the changes and check if the tests are passed.

```sh
NO_COLOR=1 cargo test --no-default-features
```

7. Make sure [rustfmt](https://github.com/rust-lang/rustfmt) and [clippy](https://github.com/rust-lang/rust-clippy) don't show any errors/warnings.

```sh
cargo fmt --all -- --check --verbose
```

```sh
cargo clippy --verbose -- -D warnings
```

8. Push changes to your fork.

9. Open a PR in our repository and follow the [PR template](./.github/PULL_REQUEST_TEMPLATE.md) so that we can efficiently review the changes.

10. Wait for approval from the repository owners. Discuss the possible changes and update your PR if necessary.

11. The PR will be merged once you have the sign-off of the repository owners.

## License

By contributing, you agree that your contributions will be licensed under [The MIT License](./LICENSE-MIT) or [Apache License 2.0](./LICENSE-APACHE).
