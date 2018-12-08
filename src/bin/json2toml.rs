use std::io::{self, BufReader, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(io::stdin());
    let mut writer = BufWriter::new(io::stdout());

    let mut buffer = String::new();

    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let mut serializer = serde_toml::Serializer::new(&mut buffer);

    serde_transcode::transcode(&mut deserializer, &mut serializer)?;
    writer.write_all(buffer.as_str().as_bytes())?;
    Ok(())
}
