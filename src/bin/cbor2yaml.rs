use std::io::{self, BufReader, BufWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = BufReader::new(io::stdin());
    let writer = BufWriter::new(io::stdout());

    let mut deserializer = serde_cbor::Deserializer::from_reader(reader);
    let mut serializer = serde_yaml::Serializer::new(writer);

    serde_transcode::transcode(&mut deserializer, &mut serializer)?;
    Ok(())
}
