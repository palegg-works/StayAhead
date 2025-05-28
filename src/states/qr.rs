use base64::engine::general_purpose;
use base64::Engine;
use image::codecs::bmp::BmpEncoder;
use image::{ColorType, ImageEncoder, Luma};
use qrcode::QrCode;

pub fn generate_qr_data_url(data: String) -> Option<String> {
    let code = QrCode::new(data).ok()?;
    let image = code.render::<Luma<u8>>().build();

    let mut buffer = Vec::new();
    let mut encoder = BmpEncoder::new(&mut buffer);
    encoder
        .encode(
            image.as_raw(),
            image.width(),
            image.height(),
            ColorType::L8.into(),
        )
        .ok()?;

    let encoded = general_purpose::STANDARD.encode(&buffer);
    Some(format!("data:image/bmp;base64,{}", encoded))
}
