use std::fs::File;
use std::io::BufReader;
use std::path::Path;
use anyhow::Result;
use exif::{In, Reader, Tag, Value};

use crate::models::ExifData;

pub struct ExifParser;

impl ExifParser {
    pub fn parse<P: AsRef<Path>>(path: P) -> Result<ExifData> {
        let file = File::open(&path)?;
        let mut bufreader = BufReader::new(file);
        let exifreader = Reader::new();

        let exif = match exifreader.read_from_container(&mut bufreader) {
            Ok(exif) => exif,
            Err(_) => return Ok(ExifData::default()),
        };

        let mut data = ExifData::default();

        if let Some(field) = exif.get_field(Tag::Make, In::PRIMARY) {
            data.camera_make = Some(field.display_value().to_string().replace('"', ""));
        }
        if let Some(field) = exif.get_field(Tag::Model, In::PRIMARY) {
            data.camera_model = Some(field.display_value().to_string().replace('"', ""));
        }
        if let Some(field) = exif.get_field(Tag::LensModel, In::PRIMARY) {
            data.lens_model = Some(field.display_value().to_string().replace('"', ""));
        }
        if let Some(field) = exif.get_field(Tag::FNumber, In::PRIMARY) {
            data.f_number = Some(format!("f/{}", field.display_value()));
        }
        if let Some(field) = exif.get_field(Tag::ExposureTime, In::PRIMARY) {
            data.exposure_time = Some(format!("{}s", field.display_value()));
        }
        if let Some(field) = exif.get_field(Tag::PhotographicSensitivity, In::PRIMARY) {
            if let Value::Short(ref v) = field.value {
                if let Some(&iso) = v.first() {
                    data.iso = Some(iso as u32);
                }
            }
        }
        if let Some(field) = exif.get_field(Tag::FocalLength, In::PRIMARY) {
            data.focal_length = Some(format!("{}mm", field.display_value()));
        }
        if let Some(field) = exif.get_field(Tag::DateTimeOriginal, In::PRIMARY) {
            data.date_time_original = Some(field.display_value().to_string().replace('"', ""));
        }

        Ok(data)
    }
}
