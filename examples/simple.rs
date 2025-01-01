
use ean_rs::is_correct_ean;

fn main() {
    println!("is 12345678 an ean ? {}",is_correct_ean("12345678"));
    println!("is 12345670 an ean ? {}",is_correct_ean("12345670"));
    println!("is 3666154117284 an ean ? {}",is_correct_ean("3666154117284"));
    ean_rs::save_as_png("3666154117284", "testResults/test13.png",None);

    let config_png = ean_rs::PngConfig
    {
        height_barcode: 200,
        border_size: 50
    };

    ean_rs::save_as_png("3666154117284", "testResults/test13custom.png",Some(config_png));
    ean_rs::save_as_svg("3666154117284", "testResults/test13.svg");
    ean_rs::save_as_png("12345670", "testResults/test8.png", None);
    ean_rs::save_as_svg("12345670", "testResults/test8.svg");
    ean_rs::show_on_terminal("12345670");
}