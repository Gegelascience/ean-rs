# EAN-RS



EAN-RS is a Rust library for handling European Article Numbers (EAN). This library provides functionality to validate and to render EAN codes.

## Features

- Validate EAN-8 and EAN-13 codes
- Generate valid EAN-8 and EAN-13 codes

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
ean-rs = "0.1.0"
```

## Usage

Here is a simple example of how to check if a string is an EAN:

```rust
extern crate ean_rs;

use ean_rs::is_correct_ean;

fn main() {
    let possible_ean13 = "3666154117284";
    println!("Is valid: {}", is_correct_ean(possible_ean13));
}
```

Show it on terminal

```rust
extern crate ean_rs;

use ean_rs::show_on_terminal;

fn main() {
    let ean13 = "3666154117284";
    show_on_terminal(ean13);
}
```


If you want to generate EAN on svg file:

```rust
extern crate ean_rs;

use ean_rs::save_as_svg;

fn main() {
    let ean13 = "3666154117284";
    save_as_svg(ean13,"path/to/svg/file");
}
```

If you want to generate EAN on png file:

```rust
extern crate ean_rs;

use ean_rs::save_as_png;

fn main() {
    let ean13 = "3666154117284";
    // simple png
    save_as_png(ean13,"path/to/png/file",None);

    // define specific dimensions
    let config_png = ean_rs::PngConfig::Simple(ean_rs::SimplePngConfig
    {
        height_barcode: 200,
        border_size: 50
    });
    save_as_png(ean13,"path/to/png/file2",Some(config_png));
}
```

## Contributing

Contributions are welcome! Please open an issue or submit a pull request.

## License

MIT

