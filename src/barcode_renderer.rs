use std::fs;

use crate::svg_parser;
use crate::png_writer;

/// Show ean on terminal
pub fn show_barcode_on_terminal(barcode_data: String) {
    for _ in 0..5 {
        let mut line: String = Default::default();
        for char in barcode_data.chars() {
            if char == '1' {
                line.push('#');
            } else {
                line.push(' ');
            }
        }
        println!("{}",line);
    }
}

/// Save Ean in an svg file
pub fn save_barcode_as_svg(barcode_data: String,file_path:&str)-> std::io::Result<()> {
    let svg_content =  svg_parser::get_svg_string(barcode_data);
        
    fs::write(file_path, svg_content.as_bytes())?;

    Ok(())
}

/// Dimensions configuration for png file
/// 
/// # Examples
/// 
/// ```
///    let config_png = ean_rs::PngConfig
///   {
///      height_barcode: 200,
///      border_size: 50
///  };
/// ```
/// 
pub struct PngConfig {
    pub height_barcode: u32,
    pub border_size: u32,
}

/// Save Ean in an png file
pub fn save_barcode_as_png(barcode_data: String,file_path:&str,config:PngConfig) {
    let mut f = std::fs::File::create(file_path).unwrap();

        let image_width = barcode_data.len() *10;
        let size_border = config.border_size;
        let image_height = config.height_barcode;
        let mut img_data: Vec<u8> = Vec::new();
        let mut line_index = 0;

        for _ in 0..size_border {
            for _ in 0..size_border {
                for _i in 0..4 {
                    img_data.push(
                        0xff
                    );
                }
            }
            for _ in barcode_data.chars() {
                for _ in 0..40 {
                    img_data.push(
                        0xff
                    );
                }
                
            }
            for _ in 0..size_border {
                for _i in 0..4 {
                    img_data.push(
                        0xff
                    );
                }
            }
        }

        while line_index < image_height {


            for _ in 0..size_border {
                for _i in 0..4 {
                    img_data.push(
                        0xff
                    );
                }
            }
            // ecriture d une ligne
            for char in barcode_data.chars() {
                if char == '1' {
                    for _n in 0..10 {
                        for _i in 0..4 {
                            img_data.push(
                                0x00
                            );
                        }
                        
                    }
                
                } else {
                    for _n in 0..10 {
                        for _i in 0..4 {
                            img_data.push(
                                0xff
                            );
                        }
                        
                    }
                
                }
    
            }
            for _ in 0..size_border {
                for _i in 0..4 {
                    img_data.push(
                        0xff
                    );
                }
            }

            line_index +=1
        }

        for _ in 0..size_border {
            for _ in 0..size_border {
                for _i in 0..4 {
                    img_data.push(
                        0xff
                    );
                }
            }
            for _ in barcode_data.chars() {
                for _ in 0..40 {
                    img_data.push(
                        0xff
                    );
                }
                
            }
            for _ in 0..size_border {
                for _i in 0..4 {
                    img_data.push(
                        0xff
                    );
                }
            }
        }
        
        match png_writer::write(&mut f, &img_data, image_width as u32 + 2*size_border, image_height + 2*size_border) {
            Ok(_) => {
                println!("Image successful saved on {}",file_path);
            },
            Err(e) => panic!("Error {:?}", e),
        }
}