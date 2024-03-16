use std::str::FromStr;

use material_colors::theme_from_source_color;
use material_colors::utils::theme::Theme;
use material_colors::Argb;
use material_colors::FilterType;
use material_colors::ImageReader;

use clap::Parser;
use tokio::fs;
use tokio::io;
use tokio::io::AsyncWriteExt;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// get color from image file
    #[arg(short, long, default_value = "")]
    file: String,

    /// result output file, default is print to stdout
    #[arg(short, long, default_value = "")]
    output: String,

    /// get color from source color
    #[arg(short, long, default_value = "")]
    source: String,

    /// is dark mode
    #[arg(long, default_value_t = false)]
    dark: bool,
}

#[tokio::main]
async fn main() -> Result<(), ()> {
    let args = Args::parse();

    let theme = if !args.file.is_empty() {
        get_color_from_file(&args.file).await
    } else if !args.source.is_empty() {
        get_color_from_source(&args.source)
    } else {
        panic!("you must input a source or a image file path!");
    };

    output_color(&theme, &args.output, args.dark).await;

    return Ok(());
}

// fn cprint(argb: &Argb, text: &str) {
//     println!(
//         "\x1b[38;2;{};{};{}m{}\x1b[0m",
//         argb.red, argb.green, argb.blue, text
//     );
// }

async fn get_color_from_file(path: &String) -> Theme {
    let image = fs::read(path).await.expect("filed to read image file!");
    let mut data = ImageReader::read(image).expect("filed to read vec image!");
    data.resize(192, 108, FilterType::Lanczos3);

    return theme_from_source_color(ImageReader::extract_color(&data), Default::default());
}

fn get_color_from_source(source: &String) -> Theme {
    return theme_from_source_color(
        Argb::from_str(source.as_str()).expect("filed to parse source color to argb!"),
        Default::default(),
    );
}

async fn output_color(theme: &Theme, output: &String, is_dark: bool) {
    let scheme = if is_dark {
        &theme.schemes.dark
    } else {
        &theme.schemes.light
    };
    let msg = format!(
        "$background={}\n$onBackground={}\n\n\
        $primary={}\n$secondary={}\n$tertiary={}\n$error={}\n\
        $primaryContainer={}\n$secondaryContainer={}\n$tertiaryContainer={}\n$errorContainer={}\n\
        $surfaceDim={}\n$surface={}\n$surfaceBright={}\n$surfaceContainer={}\n\
        $outline={}\n$shadow={}\n\
        $inversePrimary={}\n$inverseSurface={}\n\n\
        $onPrimary={}\n$onSecondary={}\n$onTertiary={}\n$onError={}\n\
        $onPrimaryContainer={}\n$onSecondaryContainer={}\n\
        $onTertiaryContainer={}\n$onErrorContainer={}\n\
        $onSurface={}\n$scrim={}\n",
        scheme.background.as_hex(), scheme.on_background.as_hex(),
        scheme.primary.as_hex(), scheme.secondary.as_hex(),
        scheme.tertiary.as_hex(), scheme.error.as_hex(),
        scheme.primary_container.as_hex(), scheme.secondary_container.as_hex(),
        scheme.tertiary_container.as_hex(), scheme.error_container.as_hex(),
        scheme.surface_dim.as_hex(), scheme.surface.as_hex(),
        scheme.surface_bright.as_hex(), scheme.surface_container.as_hex(),
        scheme.outline.as_hex(), scheme.shadow.as_hex(),
        scheme.inverse_primary.as_hex(), scheme.inverse_surface.as_hex(),
        scheme.on_primary.as_hex(), scheme.on_secondary.as_hex(),
        scheme.on_tertiary.as_hex(), scheme.on_error.as_hex(),
        scheme.on_primary_container.as_hex(), scheme.on_secondary_container.as_hex(),
        scheme.on_tertiary_container.as_hex(), scheme.on_error_container.as_hex(),
        scheme.on_surface.as_hex(), scheme.scrim.as_hex()
    );
    if output.is_empty() {
        // print color to stdout
        println!("{}", msg);
    } else {
        // print color to a file
        let file = fs::File::create(output).await.expect(&format!("open file: {} error!", output.as_str()));
        let mut buf = io::BufWriter::new(file);
        buf.write_all(msg.as_bytes()).await.expect("write color to file error!");
        buf.flush().await.expect("");
    }
}
