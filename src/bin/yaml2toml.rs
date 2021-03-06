use std::io::{self, BufWriter, Write};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = io::stdin();
    let reader = reader.lock();
    let mut writer = BufWriter::new(io::stdout());

    let mut buffer = String::new();

    let deserializer = serde_yaml::Deserializer::from_reader(reader);
    let mut serializer = serde_toml::Serializer::new(&mut buffer);

    serde_transcode::transcode(deserializer, &mut serializer)?;
    writer.write_all(buffer.as_str().as_bytes())?;
    Ok(())
}
