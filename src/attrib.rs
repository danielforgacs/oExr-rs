enum AttrType {
    /// exr header attributes for all files:
    /// <https://openexr.com/en/latest/OpenEXRFileLayout.html#header-attributes-all-files>
    Channels,
    Compression,
    DataWindow,
    DisplayWindow,
    LineOrder,
    PixelAspectRatio,
    ScreenWindowCenter,
    ScreenWindowWidth,
    String(String),
}

impl From<String> for AttrType {
    fn from(value: String) -> Self {
        match value.as_ref() {
            "channels" => Self::Channels,
            "compression" => Self::Compression,
            "dataWindow" => Self::DataWindow,
            "displayWindow" => Self::DisplayWindow,
            "lineOrder" => Self::LineOrder,
            "pixelAspectRatio" => Self::PixelAspectRatio,
            "screenWindowCenter" => Self::ScreenWindowCenter,
            "screenWindowWidth" => Self::ScreenWindowWidth,
            _ => Self::String(value),
        }
    }
}

impl AttrType {
    fn bytes(&self) -> impl Iterator<Item = u8> +'_ {
        match self {
            Self::Channels => "channels".bytes(),
            Self::Compression => "compression".bytes(),
            Self::DataWindow => "dataWindow".bytes(),
            Self::DisplayWindow => "displayWindow".bytes(),
            Self::LineOrder => "lineOrder".bytes(),
            Self::PixelAspectRatio => "pixelAspectRatio".bytes(),
            Self::ScreenWindowCenter => "screenWindowCenter".bytes(),
            Self::ScreenWindowWidth => "screenWindowWidth".bytes(),
            Self::String(text) => text.as_str().bytes(),
        }
    }
}

pub struct Attribute {
    name: String,
    attrtype: AttrType,
    len: u32,
    value: Vec<u8>,
}

impl Attribute {
    pub fn new(name: String, attrtype: String, len: u32, value: Vec<u8>) -> Self {
        Self { name, attrtype: attrtype.into(), len, value }
    }

    pub fn deserialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend(self.name.bytes());
        data.push(0);
        data.extend(self.attrtype.bytes());
        data.push(0);
        data.extend(self.len.to_le_bytes());
        data.extend(self.value.clone());
        data
    }
}
