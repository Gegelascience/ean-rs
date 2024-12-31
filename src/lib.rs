mod ean_checker;
mod barcode_renderer;
mod svg_parser;
mod png_writer;
mod barcode_data;

pub use crate::ean_checker::is_correct_ean;

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