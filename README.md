# oneocr-rs

[![Crates.io](https://img.shields.io/crates/v/oneocr-rs.svg)](https://crates.io/crates/oneocr-rs)
[![Docs.rs](https://docs.rs/oneocr-rs/badge.svg)](https://docs.rs/oneocr-rs)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A Rust binding for OneOCR, the OCR engine embedded in the Windows 11 Snipping Tool app ✂️.

This crate allows you to leverage the powerful OCR capabilities of the Windows 11 Snipping Tool within your Rust applications. It provides a simple and efficient way to perform OCR on images, extract text, and obtain bounding boxes for lines and words.

## ✨ Features

-   🖼️ Perform OCR on images.
-   📏 Get bounding boxes for lines and words.
-   💯 Get confidence scores for words.
-   📐 Get image angle.
-   ✍️ Distinguish between handwritten and printed text.
-   ⚙️ Configure OCR options (e.g., max line count, resize resolution).

## 📋 Prerequisites

-   💻 Windows 11 (not tested on Windows 10, as it may not work).
-   📄 The `oneocr.dll`, `oneocr.onemodel`, and `onnxruntime.dll` files must be present in the same directory as your executable. These files are part of the Snipping Tool app. You can find its installation location by running the following PowerShell command. After locating the folder, copy these three files into your project's target directory (e.g., `target/debug` or `target/release`) or alongside your final executable.
```powershell
Get-AppxPackage Microsoft.ScreenSketch | Select-Object -ExpandProperty InstallLocation
# Example output: 
# C:\Program Files\WindowsApps\Microsoft.ScreenSketch_11.2504.38.0_x64__8wekyb3d8bbwe
```

## 🚀 Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
oneocr-rs = "0.1.0" # Replace with the latest version
```

## 💡 Usage

```rust
use oneocr_rs::{OcrEngine, OneOcrError};
use std::path::Path;

fn main() -> Result<(), OneOcrError> {
    // Create a new OCR engine instance
    let ocr_engine = OcrEngine::new()?;

    // Replace with your image path
    let image_path = Path::new("screenshot.png"); 

    // Perform OCR on an image
    let ocr_result = ocr_engine.run(image_path, false)?;

    // Print the OCR lines and their bounding boxes
    for line in &ocr_result.lines {
        println!("Text: {}, Bounding Box: {}", line.text, line.bounding_box);
    }

    Ok(())
}
```

See the [examples](examples) directory for more detailed usage examples.

## 🖼️ Showcase
<img src="https://raw.githubusercontent.com/wangfu91/oneocr-rs/master/assets/bbox_draw.jpg" height="240" alt="Bounding box draw of OCR result" />

## 🙌 Contributing

Contributions are welcome! Please feel free to submit a pull request or open an issue if you have suggestions or find bugs.

## 🙏 Acknowledgements

This project is based on the excellent work done by `b1tg` in the [b1tg/win11-oneocr](https://github.com/b1tg/win11-oneocr) repository. Their efforts in reverse-engineering and understanding the OneOCR interface made this Rust binding possible.

## 📜 License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.
