enum FormatVersion {
    Two,
}

enum ExrType {
    ScanLineSinglePart,
}

pub struct VersionField {
    format_version_number: FormatVersion,
    exr_type: ExrType,
}

impl FormatVersion {
    fn to_byte(&self) -> u8 {
        match self {
            Self::Two => 2
        }
    }
}

impl ExrType {
    fn serialize(&self) -> Vec<u8> {
        match self {
            ExrType::ScanLineSinglePart => vec![0, 0, 0],
        }
    }
}

impl VersionField {
    pub fn new() -> Self {
        Self {
            format_version_number: FormatVersion::Two,
            exr_type: ExrType::ScanLineSinglePart,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut bytes = vec![];
        bytes.push(self.format_version_number.to_byte());
        bytes.extend(self.exr_type.serialize());
        bytes
    }
}
