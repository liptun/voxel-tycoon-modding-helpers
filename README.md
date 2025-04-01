
# Voxel Tycoon Modding Helpers

A set of simple CLI tools for Voxel Tycoon content creators.

## Features
- Export images from `*.obj.meta` files for previewing colors in 3D software (from the asset editor).

## Usage
```bash
vt-utils export my-model.obj.meta ~/Desktop -a
```
All available options are described in help accesible with
```bash
vt-utils help
```

## Installation

There is currently no pre-compiled executable for macOS, Linux, or Windows. To use this tool, you need to compile it from source.

### Prerequisites:
- Ensure that Rust is installed on your machine.

### Manual Compilation:
1. Run the following to build the executable:
   ```bash
   cargo build --release
   ```
2. Copy the compiled binary from `target/release/vt-utils` to a directory included in your system's PATH.

### Automated Installation (macOS/Linux):
If you're on macOS or Linux, you can simplify the build and installation process using `cargo-make`:
1. Install `cargo-make`.
2. Run:
   ```bash
   cargo make release
   ```
   
### macOS Codesigning:
To codesign the binary on macOS, use:
```bash
cargo make release-codesign
```
Make sure the `$DEVELOPER_ID` environment variable is set to your developer identity.

## Changelog

### 1.0.0
- Initial release
- `export` subcommand for generating images from `*.obj.meta` files

### 1.1.0
- Added support for exporting color palette variants.
- Introduced `--variant <name>` parameter under the `export` subcommand to specify the variant's name.
