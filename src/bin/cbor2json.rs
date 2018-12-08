use std::io::{self, BufReader, BufWriter};

fn main() {
    let reader = BufReader::new(io::stdin());
    let writer = BufWriter::new(io::stdout());

    let mut deserializer = serde_cbor::Deserializer::from_reader(reader);
    let mut serializer = serde_json::Serializer::new(writer);

    serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();
}
