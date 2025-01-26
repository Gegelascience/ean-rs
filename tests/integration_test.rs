
#[test]
fn it_save_png() {

    ean_rs::save_as_png("3666154117284", "test13.png",None);
    let config_png = ean_rs::PngConfig::Simple(ean_rs::SimplePngConfig {
        height_barcode: 200,
        border_size: 50
    });

    ean_rs::save_as_png("3666154117284", "test13custom.png",Some(config_png));

    let config_red_png = ean_rs::PngConfig::Advanced(ean_rs::AdvancedPngConfig {
        height_barcode: 200,
        border_size: 50,
        color_barcode: (255,0,0),
    });
      
    ean_rs::save_as_png("3666154117284", "test13custom_red.png",Some(config_red_png));
    ean_rs::save_as_png("12345670", "test8.png", None);
}

#[test]
fn it_fail_save_png() {
    let result = std::panic::catch_unwind(|| ean_rs::save_as_png("3666154117284", "someDir/test13.png",None));
    assert!(result.is_err());
}

#[test]
fn it_save_svg() {
    ean_rs::save_as_svg("3666154117284", "test13.svg");
    ean_rs::save_as_svg("12345670", "test8.svg");
}

#[test]
fn it_show_on_terminal() {
    ean_rs::show_on_terminal("3666154117284");
    ean_rs::show_on_terminal("12345670");
}