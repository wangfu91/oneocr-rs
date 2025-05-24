use std::{path::Path, time::Duration};

use criterion::{Criterion, criterion_group, criterion_main};
use oneocr_rs::OneOcrError;

pub fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("ocr_bench_group");
    group.measurement_time(Duration::from_secs(60));
    group.sample_size(10);
    group.warm_up_time(Duration::from_secs(10));
    group.bench_function("ocr_simple", |b| b.iter(ocr_simple));
    group.bench_function("ocr_advance", |b| b.iter(ocr_advance));
    group.finish();
}

#[inline]
pub fn ocr_simple() -> Result<(), OneOcrError> {
    // Create a new OCR instance
    let ocr_engine = oneocr_rs::OcrEngine::new()?;

    // Perform OCR on an image
    let image_path = Path::new("./assets/sample.jpg");
    let _ocr_result = ocr_engine.run(image_path, false)?;

    Ok(())
}

#[inline]
pub fn ocr_advance() -> Result<(), OneOcrError> {
    // Create a new OCR instance
    let ocr_engine = oneocr_rs::OcrEngine::new()?;

    // Perform OCR on an image
    let image_path = Path::new("./assets/sample.jpg");
    let _ocr_result = ocr_engine.run(image_path, true)?;

    Ok(())
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
