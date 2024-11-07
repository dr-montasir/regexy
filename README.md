<div style="text-align: center;">
  <a href="https://crates.io/crates/regexy"><img src="logo.svg" alt="LOGO" /></a>
</div>

<div style="text-align: center;">
  <a href="https://github.com/dr-montasir/regexy"><img src="https://img.shields.io/badge/github-dr%20montasir%20/%20regexy-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="24" style="margin-top: 10px;" alt="github" /></a> <a href="https://crates.io/crates/regexy"><img src="https://img.shields.io/crates/v/regexy.svg?style=for-the-badge&color=fc8d62&logo=rust" height="24" style="margin-top: 10px;" alt="crates.io"></a> <a href="https://docs.rs/regexy"><img src="https://img.shields.io/badge/docs.rs-regexy-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="24" style="margin-top: 10px;" alt="docs.rs"></a> <a href="https://choosealicense.com/licenses/apache-2.0"><img src="https://img.shields.io/badge/license-apache_2.0-4a98f7.svg?style=for-the-badge&labelColor=555555&logo=apache" height="24" style="margin-top: 10px;" alt="license"></a> <a href="https://choosealicense.com/licenses/mit"><img src="https://img.shields.io/badge/license-mit-4a98f7.svg?style=for-the-badge&labelColor=555555" height="24" style="margin-top: 10px;" alt="license"></a>
</div>

# REGEXY

**REGEXY** provides a simple and lightweight Rust library for working with regular expressions. The `regexy` crate provides an easy-to-use interface for matching patterns in strings using regex.

---

## Table of Contents

- [Installation](#installation)
- [Changelog](#changelog)
- [Features](#features)
- [Usage](#usage)
- [License](#license)
- [Contributing](#contributing)
- [Author](#author)

## Installation

**Run the following Cargo command in your project directory:**

```terminal
cargo add regexy
```

**Or add `regexy` to your `Cargo.toml` file:**

```toml
[dependencies]
regexy = "MAJOR.MINOR.PATCH" # Replace with the latest version
```

## Changelog

[![github](https://img.shields.io/badge/github-%20changelog-8da0cb?style=for-the-badge&labelColor=555555&logo=github)](https://github.com/dr-montasir/regexy/blob/main/CHANGELOG.md)

## Features

- Check if a string matches a regex pattern.
- Easy integration with the `regex` crate.
- Simple API for common regex operations.

## Usage

```rust
use regexy::utils::is_match;

fn main() {
    let pattern = r"\d+";
    let text = "There are 123 apples.";
    // let text = "There are three apples.";
    
    if is_match(pattern, text) {
        println!("The text contains digits!");
    } else {
        println!("No digits found in the text.");
    }
}
```

### Result

```shell
The text contains digits!

// No digits found in the text.
```

## License

This project is licensed under either of the following licenses:

- MIT License
- Apache License, Version 2.0

You may choose either license for your purposes.

## Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue for any feature requests or bug reports.

## Author

[Dr. Montasir Mirghani](https://github.com/dr-montasir)
