mod aseprite;

fn main() {
    // input/sample.asepriteを読み込んで、FileHeaderをパースする
    let input = include_bytes!("../input/sample.aseprite");
    let (_, file_header) = aseprite::read_aseprite_file_header(input).unwrap();
    println!("{:?}", file_header);
}
