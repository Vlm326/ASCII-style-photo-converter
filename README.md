# ASCII_photo_converter 🦀

[![CI](https://github.com/Vlm326/ASCII-style-photo-converter/actions/workflows/rust.yml/badge.svg)](https://github.com/Vlm326/ASCII-style-photo-converter/actions/workflows/rust.yml)
[![License](https://img.shields.io/github/license/Vlm326/ASCII-style-photo-converter)](LICENSE)

A fast Rust CLI tool that converts photographs into ASCII-art images. Each grid cell of the source image is replaced by a rendered glyph chosen from a configurable character set, colored with the average color of that region. The result is a pixel-art-like PNG made of letters and symbols.

## Features

- Converts a wide range of image formats (PNG, JPG, GIF, TIFF, WebP, AVIF, and more).
- Configurable output resolution (`cols`) with aspect-ratio-preserving row calculation.
- Customizable character set and glyph font.
- Renders each glyph in its local region color for a rich, photo-realistic result.
- Simple command-line interface with optional interactive prompts.
- Batch conversion helper script included.

## Requirements

- [Rust](https://rustup.rs/) (edition 2024)
- A monospace TTF font installed on the system (see [Font setup](#font-setup)).
- Python 3.x (optional, only for the helper scripts).

## Installation

Clone the repository and build:

```bash
git clone https://github.com/Vlm326/ASCII-style-photo-converter.git
cd ASCII_photo_converter

cargo build --release
```

The binary will be produced at `target/release/ASCII_photo_converter`.

## Usage

```
ASCII_photo_converter <path> [--key=value ...]
```

If no path is provided, the tool asks for one interactively. If no options are
given, it prompts for settings — type `def` to use the defaults.

### Basic example

```bash
./target/release/ASCII_photo_converter exemple.jpg
```

By default this writes `exemple_ascii.png` in the current directory.

### Options

| Option     | Default                                                                    | Description                                                                                              |
|------------|----------------------------------------------------------------------------|----------------------------------------------------------------------------------------------------------|
| `cols`     | `220`                                                                      | Number of output columns. Rows are derived proportionally. Higher values give more detail but look darker. |
| `charset`  | a long luminance ramp of printable ASCII + block glyphs                   | The character pool used to map luminance.                                                               |
| `font`     | a hardcoded path to a JetBrains Mono Nerd Font                            | Path to the TTF font used for rendering.                                                               |
| `font_px`  | `14.0`                                                                     | Font pixel size.                                                                                          |
| `out_name` | `./<input-base-name>_ascii.png`                                            | Base output name. The `_ascii.png` suffix is always appended.                                             |

### Examples

```bash
# 100 columns, a custom charset, and a custom output name
./target/release/ASCII_photo_converter exemple.jpg \
  --cols=100 \
  --charset=" 90#M%@" \
  --out_name=my_art
# -> my_art_ascii.png
```

```bash
# Specify a font explicitly
./target/release/ASCII_photo_converter image.png --font=/path/to/Mono.ttf
```

### Font setup

By default `font` points to a Linux-specific path that only exists on the
author's machine. On any other system, either:

- pass `--font=/path/to/your.ttf` on every run, or
- edit the default path in `src/main.rs` before building.

### Interactive mode

```bash
./target/release/ASCII_photo_converter
# Enter path to file: exemple.jpg
# Enter settings or enter def if you want default: def
```

## Helper scripts

- `scripts/build.py` — builds the project and can rewrite the default font path in `src/main.rs` to your local monospace font.
- `scripts/auto_multi_convert.py` — batch-converts every file placed in the `convert/` folder, applying shared settings, and writes outputs to `out/`.

## Tips

- For bright images, prefer a coarse character set (e.g. `" 90#M%@"`), because the default settings are tuned for darker images and may render light photos too dark.
- More `cols` means sharper detail but tends to produce darker output — experiment to find the sweet spot.

## Examples

Original image and outputs at different `cols` values (uploaded to the repo's release assets) are shown in the [examples](#examples) section of the project. Additional samples with various settings live in the `exemples/` folder.

## Project structure

```
ASCII_photo_converter/
├── src/                  # Rust sources
│   ├── main.rs           # CLI entry point, option parsing, config
│   ├── converter.rs      # Core conversion algorithm
│   └── letter_pool.rs    # Luminance → character mapping
├── scripts/              # Python helper scripts
├── convert/              # Staging folder for batch conversion
├── out/                  # Batch conversion outputs
├── exemples/             # Sample inputs and outputs
├── Cargo.toml            # Rust project configuration
└── .github/workflows/    # CI configuration
```

## CI

A GitHub Actions workflow (`rust.yml`) builds the project with `--locked` on
`ubuntu-latest` for every push and pull request to `main`.

## License

[Apache 2.0](LICENSE)

## Contributing

Found a bug or have an idea? Open an [issue](https://github.com/Vlm326/ASCII-style-photo-converter/issues) or submit a pull request.