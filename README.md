# 🦀 oneocr-rs

[![Crates.io](https://img.shields.io/crates/v/oneocr-rs.svg)](https://crates.io/crates/oneocr-rs)
[![Docs.rs](https://docs.rs/oneocr-rs/badge.svg)](https://docs.rs/oneocr-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust 🦀 binding for OneOCR, the embedded OCR engine in Windows 11 Snipping Tool ✂️.

This crate allows you to use the powerful OCR capabilities of Windows 11 in your Rust applications.

## ✨ Features

-   🖼️ Perform OCR on images.
-   📏 Get bounding boxes for lines and words.
-   💯 Get confidence scores for words.
-   📐 Get image angle.
-   ✍️ Distinguish between handwritten and printed text.
-   ⚙️ Configure OCR processing options like max recognition line count and resize resolution.

## 📋 Prerequisites

-   💻 Windows 11 (as the OCR engine is part of the OS).
-   📄 The `oneocr.dll` and `oneocr.onemodel` files must be present in the same directory as your executable or in a directory specified in your system's PATH environment variable. These files are typically found in the Windows Snipping Tool application directory (e.g., `C:\Program Files\WindowsApps\Microsoft.ScreenSketch_11.2309.16.0_x64__8wekyb3d8bbwe\`).

## 🚀 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
oneocr-rs = "0.1.0" # Replace with the latest version
```

## 💡 Usage

```rust
use oneocr_rs::errors::OneOcrError;
use std::path::Path;

fn main() -> Result<(), OneOcrError> {
    // Create a new OCR engine instance
    let ocr_engine = oneocr_rs::OcrEngine::new()?;

    let image_path = Path::new("./target/screenshot.png"); // Replace with your image path

    // Perform OCR on an image
    let ocr_result = ocr_engine.run(image_path, false)?;

    // Print the OCR result
    for line in &ocr_result.lines {
        println!("{}", line.text);
    }

    Ok(())
}
```

See the [examples](examples) directory for more detailed usage.

## 🙌 Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue.

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
