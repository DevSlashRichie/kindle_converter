use wkhtmltopdf::{PdfApplication, Error, Size, Orientation};
use std::io::Read;

pub fn convert(title: String, source: String) -> Result<Vec<u8>, Error> {
    let pdf = PdfApplication::new()?;

    let mut builder = pdf.builder()
        .title(&title)
        .orientation(Orientation::Portrait)
        .outline(None)
        .margin(Size::Millimeters(0))
        .build_from_html(source)?;

    let mut output = Vec::new();

    builder.read_to_end(&mut output)?;

    Ok(output)
}