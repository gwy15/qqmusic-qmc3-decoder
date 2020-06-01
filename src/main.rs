use std::fs::File;
use std::path::PathBuf;

mod utils;

struct ParseResult {
    pub filename: String,
    pub pin: PathBuf,
    pub pout: PathBuf,
}
impl ParseResult {
    fn parse(pin: PathBuf) -> Option<Self> {
        let filename = pin.file_name()?.to_str()?.to_owned();
        let ext = pin.extension()?;
        let converted_ext = match ext.to_str()? {
            "qmc3" => "mp3",
            "qmcflac" => "flac",
            ext => panic!(format!("Unknown extension: {}", ext)),
        };
        let pout = pin.with_extension(converted_ext);
        Some(ParseResult {
            filename,
            pin,
            pout,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    for entry in glob::glob("*.qmc*")? {
        let parsed_result = ParseResult::parse(entry?).unwrap();

        let fin = File::open(parsed_result.pin)?;
        let fout = File::create(parsed_result.pout)?;
        utils::decode_file(fin, fout)?;
        println!("{} converted", parsed_result.filename);
    }
    Ok(())
}
