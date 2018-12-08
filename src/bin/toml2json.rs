use std::io::{self, BufReader, BufWriter, Read};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut reader = BufReader::new(io::stdin());
    let writer = BufWriter::new(io::stdout());

    let mut buffer = String::new();
    reader.read_to_string(&mut buffer)?;

    let mut deserializer = serde_toml::Deserializer::new(&buffer);
    let mut serializer = serde_json::Serializer::new(writer);

    serde_transcode::transcode(&mut deserializer, &mut serializer)?;
    Ok(())
}
