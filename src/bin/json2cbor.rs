use std::io::{self, BufWriter};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reader = io::stdin();
    let reader = reader.lock();
    let writer = BufWriter::new(io::stdout());

    let writer = serde_cbor::ser::IoWrite::new(writer);

    let mut deserializer = serde_json::Deserializer::from_reader(reader);
    let mut serializer = serde_cbor::Serializer::new(writer);

    serde_transcode::transcode(&mut deserializer, &mut serializer)?;
    Ok(())
}
