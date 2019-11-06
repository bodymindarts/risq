use bytes::*;
use prost::{encoding, DecodeError, Message};

#[derive(Debug, Clone, Default, PartialEq)]
pub struct JavaStringMapEntry(String, String);

// Java protobuf lib always serializes key and value in map fields
// Prost skips serializing value if it == the default ("" for string)
impl Message for JavaStringMapEntry {
    fn encode_raw<B>(&self, buf: &mut B)
    where
        B: BufMut,
        Self: Sized,
    {
        encoding::string::encode(1, &self.0, buf);
        // Force serializing the value even if empty ("")
        encoding::string::encode(2, &self.1, buf)
    }

    fn encoded_len(&self) -> usize {
        encoding::string::encoded_len(1, &self.0) + encoding::string::encoded_len(2, &self.1)
    }

    fn merge_field<B>(&mut self, buf: &mut B) -> Result<(), DecodeError>
    where
        B: Buf,
        Self: Sized,
    {
        let (tag, wire_type) = encoding::decode_key(buf)?;
        match tag {
            1 => encoding::string::merge(wire_type, &mut self.0, buf),
            2 => encoding::string::merge(wire_type, &mut self.1, buf),
            _ => encoding::skip_field(wire_type, buf),
        }
    }

    fn clear(&mut self) {
        self.0.clear();
        self.1.clear();
    }
}
