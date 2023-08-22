/*
Header Attributes (All Files)
    channels                chlist
    compression             compression
    dataWindow              box2i
    displayWindow           box2i
    lineOrder               lineOrder
    pixelAspectRatio        float
    screenWindowCenter      v2f
    screenWindowWidth       float
Tile Header Attribute
    tiles                   tiledesc
Multi-View Header Attribute
    view                    text
Multi-Part and Deep Data Header Attributes
    name                    string
    type                    string
    version                 int
    chunkCount              int
    tiles                   tileDesc
Deep Data Header Attributes
    tiles                   tileDesc
    maxSamplesPerPixel      int
    version                 int
    type                    string
*/

pub struct Attribute {
    name: String,
    attrtype: String,
    len: u32,
    value: Vec<u8>,
}

impl Attribute {
    pub fn new(name: String, attrtype: String, len: u32, value: Vec<u8>) -> Self {
        Self {
            name,
            attrtype,
            len,
            value,
        }
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
