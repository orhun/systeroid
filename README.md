<a href="https://github.com/orhun/systeroid">
    <img align="left" src="img/systeroid-logo.jpg" width="256">
</a>

#### **`systeroid`** — A more powerful alternative to sysctl.

[`sysctl(8)`](https://man7.org/linux/man-pages/man8/sysctl.8.html) is a utility on Unix-like operating systems that is used to read and modify the attributes of the kernel such as its version number, maximum limits, and security settings[\*](https://en.wikipedia.org/wiki/Sysctl). **systeroid** is "_sysctl on steroids_". It can do everything that sysctl does and even more. It provides a safer, more performant, and user-friendly CLI/TUI for managing the kernel parameters at runtime.

<a href="https://github.com/orhun/systeroid/releases">
    <img src="https://img.shields.io/github/v/release/orhun/git-cliff?style=flat&logo=GitHub%20Actions&labelColor=000000&color=CECDCB&logoColor=CECDCB">
</a>
<a href="https://crates.io/crates/git-cliff/">
    <img src="https://img.shields.io/crates/v/git-cliff?style=flat&logo=Rust&labelColor=000000&color=CECDCB&logoColor=CECDCB">
</a>
<a href="https://codecov.io/gh/orhun/systeroid">
    <img src="https://img.shields.io/codecov/c/gh/orhun/systeroid?style=flat&logo=Codecov&labelColor=000000&color=CECDCB&logoColor=CECDCB">
</a>
<br>
<a href="https://github.com/orhun/systeroid/actions?query=workflow%3A%22Continuous+Integration%22">
    <img src="https://img.shields.io/github/workflow/status/orhun/systeroid/Continuous%20Integration?style=flat&logo=GitHub%20Actions&label=build&labelColor=000000&color=CECDCB&logoColor=CECDCB">
</a>
<a href="https://github.com/orhun/systeroid/actions?query=workflow%3A%22Continuous+Deployment%22">
    <img src="https://img.shields.io/github/workflow/status/orhun/git-cliff/Continuous%20Deployment?style=flat&logo=GitHub%20Actions&label=deploy&labelColor=000000&color=CECDCB&logoColor=CECDCB">
</a>
<a href="https://hub.docker.com/r/orhunp/git-cliff">
    <img src="https://img.shields.io/docker/cloud/build/orhunp/git-cliff?style=flat&logo=Docker&label=docker&labelColor=000000&color=CECDCB&logoColor=CECDCB">
</a>
<a href="https://docs.rs/git-cliff-core/">
    <img src="https://img.shields.io/docsrs/git-cliff-core?style=flat&logo=Rust&labelColor=000000&color=CECDCB&logoColor=CECDCB">
</a>
<br>
<br>
<br>

**systeroid** is implemented using [procfs](https://en.wikipedia.org/wiki/Procfs) which is the virtual file system that is typically mapped to a mount point named `/proc` at boot time. This means checking the value of some kernel parameter requires opening a file in this virtual filesystem, reading its contents, parsing them, and closing the file. In Linux, these dynamically configurable kernel options are available under `/proc/sys` which contains directories representing the sections of the kernel and readable/writable virtual files. For example, to enable/disable IP forwarding, `1` or `0` could be written in `/proc/sys/net/ipv4/ip_forward` or `systeroid ip_forward=1` command can be used to change the value of the parameter.

<details>
  <summary>Table of Contents</summary>

- [Usage](#usage)
  - [Options](#options)
  - [Examples](#examples)
- [TUI](#tui)
  - [Usage](#usage-1)
  - [Key Bindings](#key-bindings)
  - [Examples](#examples-1)
- [Resources](#resources)
  - [Logo](#logo)
  - [Social Links](#social-links)
  - [Funding](#funding)
- [Contributing](#contributing)
- [License](#license)
- [Copyright](#copyright)

</details>

## Usage

```
systeroid [options] [variable[=value] ...] --load[=<file>]
```

### Options

```
-a, --all           display all variables (-A,-X)
-T, --tree          display the variables in a tree-like format
-J, --json          display the variables in JSON format
    --deprecated    include deprecated variables while listing
-e, --ignore        ignore unknown variable errors
-N, --names         print only variable names
-n, --values        print only variable values
-b, --binary        print only variable values without new line
-p, --load          read values from file (-f)
-S, --system        read values from all system directories
-r, --pattern <expr>
                    use a regex for matching variable names
-q, --quiet         do not print variable after the value is set
-w, --write         only enable writing a value to variable
-E, --explain       provide a detailed explanation for variable
-D, --docs <path>   set the path of the kernel documentation
-P, --no-pager      do not pipe output into a pager
-v, --verbose       enable verbose logging
    --tui           show terminal user interface
-h, --help          display this help and exit (-d)
-V, --version       output version information and exit
```

Most of the arguments/flags are inherited from `sysctl` so they have the same functionality.

### Examples

TBA

## TUI

### Usage

```
systeroid-tui [options]
```

```
-t, --tick-rate <ms>
                    set the tick rate of the terminal [default: 250]
-D, --docs <path>   set the path of the kernel documentation
-s, --section <section>
                    set the section to filter
-q, --query <query> set the query to search
    --bg-color <color>
                    set the background color [default: black]
    --fg-color <color>
                    set the foreground color [default: white]
-n, --no-docs       do not show the kernel documentation
-h, --help          display this help and exit
-V, --version       output version information and exit
```

### Key Bindings

| Key                             | Action                       |
| ------------------------------- | ---------------------------- |
| `?`, `f1`                       | show help                    |
| `up/down`, `k/j`, `pgup/pgdown` | scroll list                  |
| `t/b`                           | scroll to top/bottom         |
| `left/right`, `h/l`             | scroll documentation         |
| `tab`, `` ` ``                  | next/previous section        |
| `:`                             | command                      |
| `/`, `s`                        | search                       |
| `enter`                         | select / set parameter value |
| `c`                             | copy to clipboard            |
| `r`, `f5`                       | refresh                      |
| `esc`                           | cancel / exit                |
| `ctrl-c/ctrl-d`                 | exit                         |

### Examples

TBA

## Resources

### Logo

**systeroid** logo was originally painted by [Ryan Tippery](https://www.ryantippery.com/about) as a part of the [Compositions](https://www.ryantippery.com/compositions/) art collection and it is put together by me using the [Filled Spots](https://www.fontspace.com/filled-spots-font-f30755) font. Shout out to Ryan for letting me use his painting for the logo! **<3**

Check out his [store](https://www.ryantippery.com/store) for a fine piece of similar art. Kudos!

### Social Links

- [![Follow @systeroid](https://img.shields.io/twitter/follow/systeroid?style=flat&&logo=twitter&labelColor=000000&color=CECDCB&logoColor=CECDCB)](https://twitter.com/systeroid)
- [![https://orhun.dev](https://img.shields.io/badge/author-orhun-000000?style=flat&logo=Rust&labelColor=000000&color=CECDCB&logoColor=CECDCB)](https://orhun.dev)
  - [![Follow @orhun](https://img.shields.io/github/followers/orhun?label=follow%20%40orhun&style=flat&logo=GitHub&labelColor=000000&color=CECDCB&logoColor=CECDCB)](https://github.com/orhun)
  - [![Follow @orhunp_](https://img.shields.io/twitter/follow/orhunp_?style=flat&logo=twitter&labelColor=000000&color=CECDCB&logoColor=CECDCB)](https://twitter.com/orhunp_)

### Funding

If you find **systeroid** and/or other projects on my [GitHub profile](https://github.com/orhun/) useful, consider [becoming a patron](https://www.patreon.com/join/orhunp)!

[![Support me on Patreon](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fshieldsio-patreon.vercel.app%2Fapi%3Fusername%3Dorhunp%26type%3Dpatrons&style=flat&logo=Patreon&labelColor=000000&color=CECDCB&logoColor=CECDCB)](https://patreon.com/join/orhunp)
[![Support me on Patreon](https://img.shields.io/endpoint.svg?url=https%3A%2F%2Fshieldsio-patreon.vercel.app%2Fapi%3Fusername%3Dorhunp%26type%3Dpledges&style=flat&logo=Patreon&labelColor=000000&color=CECDCB&logoColor=CECDCB&label=)](https://patreon.com/join/orhunp)

## Contributing

See our [Contribution Guide](./CONTRIBUTING.md) and please follow the [Code of Conduct](./CODE_OF_CONDUCT.md) in all your interactions with the project.

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache 2.0 License, shall be dual licensed as above, without any additional terms or conditions.

## License

Licensed under either of [Apache License Version 2.0](http://www.apache.org/licenses/LICENSE-2.0) or [The MIT License](http://opensource.org/licenses/MIT) at your option.

## Copyright

Copyright © 2022, [Orhun Parmaksız](mailto:orhunparmaksiz@gmail.com)
