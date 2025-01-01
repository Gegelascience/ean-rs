//! # ean-rs
//!
//! `ean_rs` is a crate for handling European Article Numbers (EAN).
//! This crate provides functionality to validate and to render EAN codes.

mod ean_checker;
mod barcode_renderer;
mod svg_parser;
mod png_writer;
mod barcode_data;

pub use crate::ean_checker::is_correct_ean;
pub use crate::barcode_renderer::PngConfig;

/// Show an ean on the terminal
/// 
/// # Examples
/// 
/// ```
/// use ean_rs::show_on_terminal;
///
/// show_on_terminal("3666154117284");
/// ```
/// 
pub fn show_on_terminal(ean: &str) {
    if ean_checker::is_correct_ean(ean) {
        if  ean.len() ==13 {
            let barcode = barcode_data::calculate_barcode_ean13(ean);
            barcode_renderer::show_barcode_on_terminal(barcode);
        }  else if ean.len() ==8 {
            let barcode = barcode_data::calculate_barcode_ean8(ean);
            barcode_renderer::show_barcode_on_terminal(barcode);
        } 
    } 
}

/// Save an ean as a png file
/// 
/// # Examples
/// 
/// ```
/// use ean_rs::save_as_png;
/// 
/// save_as_png("3666154117284", "testResults/test13.png",None);
/// 
/// ```
/// 
/// ```	
/// use ean_rs::PngConfig;
/// use ean_rs::save_as_png;
/// 
/// let config_png = PngConfig
/// {
///    height_barcode: 200,
///   border_size: 50
/// };
/// 
/// save_as_png("3666154117284", "testResults/test13custom.png",Some(config_png));
/// 
/// ```
/// 
pub fn save_as_png(ean: &str, file_path:&str, config:Option<barcode_renderer::PngConfig>) {
    if ean_checker::is_correct_ean(ean) {
        let config_png: barcode_renderer::PngConfig;
        match config {
            Some(x) => config_png = x,
            None => config_png = barcode_renderer::PngConfig {
                height_barcode:100,
                border_size:10
            }
        }

        if  ean.len() ==13 {
            let barcode = barcode_data::calculate_barcode_ean13(ean);
            let _ = barcode_renderer::save_barcode_as_png(barcode,file_path,config_png);
        } else if ean.len() ==8 {
            let barcode = barcode_data::calculate_barcode_ean8(ean);
            let _ = barcode_renderer::save_barcode_as_png(barcode,file_path,config_png);
        }
        
    } 
}

/// Save an ean as a svg file
/// 
/// # Examples
/// 
/// ```
/// use ean_rs::save_as_svg;
/// 
/// save_as_svg("3666154117284", "testResults/test13.svg");
/// 
/// ```
/// 
pub fn save_as_svg(ean: &str, file_path:&str) {
    if ean_checker::is_correct_ean(ean) {
        if  ean.len() ==13 {
            let barcode = barcode_data::calculate_barcode_ean13(ean);
            let _ = barcode_renderer::save_barcode_as_svg(barcode,file_path);
        }  else if ean.len() ==8 {
            let barcode = barcode_data::calculate_barcode_ean8(ean);
            let _ = barcode_renderer::save_barcode_as_svg(barcode,file_path);
        } 
    } 
}