use nom::{CompareResult::Error, Err, IResult, Parser, bytes::complete::take, character::complete::alpha0, multi::{many, many0}, number::complete::le_u32};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct RecordType([u8; 4]);

impl From<[u8; 4]> for RecordType {
    fn from(bytes: [u8; 4]) -> Self {
        RecordType(bytes)
    }
}

impl TryFrom<&[u8]> for RecordType {
    type Error = ();

    fn try_from(bytes: &[u8]) -> Result<Self, Self::Error> {
        if bytes.len() < 4 {
            return Err(());
        }

        let mut array = [0u8; 4];
        array.copy_from_slice(&bytes[0..4]);
        Ok(RecordType(array))
    }
}

impl std::fmt::Debug for RecordType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match std::str::from_utf8(self.0.as_ref()) {
            Ok(s) => write!(f, "{}", s),
            Err(_) => write!(f, "{:?}", self),
        }
    }
}

// Define Record Flags for bitwise verification
pub const FLAG_PERSISTENT: u32 = 0x0000_0400;
pub const FLAG_BLOCKED: u32 = 0x0000_2000;

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct RecordHeader {
    pub record_type: RecordType,
    pub flags: u32,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct SubRecord {
    pub record_type: RecordType,
    pub data: Vec<u8>,
}

#[derive(Clone, PartialEq, Eq, Debug, Hash, Default)]
pub struct Record {
    pub header: RecordHeader,
    pub subrecords: Vec<SubRecord>,
}

fn parse_subrecord(input: &[u8]) -> IResult<&[u8], SubRecord> {
    let (input, name_bytes) = take(4usize)(input)?;
    let (input, size) = le_u32(input)?;
    let (input, data) = take(size)(input)?;

    let subrecord = SubRecord {
        record_type: name_bytes.try_into().unwrap(),
        data: data.to_vec(),
    };

    Ok((input, subrecord))
}

fn parse_record(input: &[u8]) -> IResult<&[u8], Record> {
    let (input, name_bytes) = take(4usize)(input)?;
    let (input, size) = le_u32(input)?;
    let (input, _header1) = le_u32(input)?;
    let (input, flags) = le_u32(input)?;

    let header = RecordHeader {
        record_type: name_bytes.try_into().unwrap(),
        flags,
    };

    let (input, subrecord_data) = take(size)(input)?;

    let (_, subrecords) = many0(parse_subrecord).parse(subrecord_data)?;

    Ok((input, Record { header, subrecords }))
}

pub fn parse_records(input: &[u8]) -> crate::error::Result<Vec<Record>> {
    Ok(many0(parse_record).parse(input).map_err(|_| crate::error::Error::ParsingError)?.1)
}

