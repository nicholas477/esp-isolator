use crate::record::RecordType;

pub trait RecordNameTrait {
    fn record_type() -> &'static crate::record::RecordType;
}

pub trait SubRecordTypeTrait: RecordNameTrait + TryFrom<crate::record::SubRecord> {}

pub trait RecordTypeTrait: RecordNameTrait + TryFrom<crate::record::Record> {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Name(String);

static NAME_TYPE: crate::record::RecordType = crate::record::RecordType([b'N', b'A', b'M', b'E']);

impl RecordNameTrait for Name {
    fn record_type() -> &'static crate::record::RecordType {
        &NAME_TYPE
    }
}

impl TryFrom<crate::record::SubRecord> for Name {
    type Error = ();

    fn try_from(subrecord: crate::record::SubRecord) -> Result<Self, Self::Error> {
        if subrecord.record_type == *Self::record_type() {
            let name_str = String::from_utf8(subrecord.data).map_err(|_| ())?;
            Ok(Name(name_str))
        } else {
            Err(())
        }
    }
}

impl SubRecordTypeTrait for Name {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Model {
    pub path: String,
}

static MODL_TYPE: crate::record::RecordType = crate::record::RecordType([b'M', b'O', b'D', b'L']);

impl RecordNameTrait for Model {
    fn record_type() -> &'static crate::record::RecordType {
        &MODL_TYPE
    }
}

impl TryFrom<crate::record::SubRecord> for Model {
    type Error = ();

    fn try_from(subrecord: crate::record::SubRecord) -> Result<Self, Self::Error> {
        if subrecord.record_type == *Self::record_type() {
            let path_str = String::from_utf8(subrecord.data).map_err(|_| ())?;
            Ok(Model { path: path_str })
        } else {
            Err(())
        }
    }
}

impl SubRecordTypeTrait for Model {}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Default)]
pub struct Static {
    pub name: Name,
    pub model: Model,
}

static STAT_TYPE: crate::record::RecordType = crate::record::RecordType([b'S', b'T', b'A', b'T']);

impl RecordNameTrait for Static {
    fn record_type() -> &'static crate::record::RecordType {
        &STAT_TYPE
    }
}

impl TryFrom<crate::record::Record> for Static {
    type Error = ();

    fn try_from(record: crate::record::Record) -> Result<Self, Self::Error> {
        if record.header.record_type == *Self::record_type() {
            let mut name_opt: Option<Name> = None;
            let mut model_opt: Option<Model> = None;

            for subrecord in record.subrecords {
                if subrecord.record_type == *Name::record_type() {
                    name_opt = Some(Name::try_from(subrecord)?);
                } else if subrecord.record_type == *Model::record_type() {
                    model_opt = Some(Model::try_from(subrecord)?);
                }
            }

            if let (Some(name), Some(model)) = (name_opt, model_opt) {
                Ok(Static { name, model })
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }
}

impl RecordTypeTrait for Static {}
