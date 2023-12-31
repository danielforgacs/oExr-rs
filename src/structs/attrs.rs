pub enum Compression {
    No,
}

// dataWindow: box2i:  4 x int: xMin, yMin, xMax, yMax
#[derive(Clone)]
pub struct DataWindow {
    x_min: i32,
    y_min: i32,
    x_max: i32,
    y_max: i32,
}

pub enum LineOrder {
    INCREASING_Y,
    DECREASING_Y,
    RANDOM_Y,
}

pub struct pixelAspectRatio {
    value: f32,
}

pub struct screenWindowCenter {
    value_x: f32,
    value_y: f32,
}

impl Compression {
    pub fn serialise(&self) -> Vec<u8> {
        let mut bytes = Vec::new();
        let value = match self {
            Self::No => 0_u8.to_le_bytes(),
        };
        bytes.extend("compression".as_bytes());
        bytes.push(0);
        bytes.extend("compression".as_bytes());
        bytes.push(0);
        bytes.extend((value.len() as i32).to_le_bytes());
        bytes.extend(value);
        bytes
    }
}

impl DataWindow {
    pub fn new(res_x: i32, res_y: i32) -> Self {
        Self {
            x_min: 0,
            y_min: 0,
            x_max: res_x - 1,
            y_max:  res_y - 1,
        }
    }

    pub fn get_res_x(&self) -> i32 {
        self.x_max - self.x_min + 1
    }

    pub fn get_res_y(&self) -> i32 {
        self.y_max - self.y_min + 1
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend("dataWindow".bytes());
        data.push(0);
        data.extend("box2i".bytes());
        data.push(0);
        data.extend({4_i32 * 4}.to_le_bytes());
        data.extend(self.x_min.to_le_bytes());
        data.extend(self.y_min.to_le_bytes());
        data.extend(self.x_max.to_le_bytes());
        data.extend(self.y_max.to_le_bytes());
        data
    }

    pub fn serialize_as_display_window(&self) -> Vec<u8> {
        let mut data = Vec::new();
        data.extend("displayWindow".bytes());
        data.push(0);
        data.extend("box2i".bytes());
        data.push(0);
        data.extend({4_i32 * 4}.to_le_bytes());
        data.extend(self.x_min.to_le_bytes());
        data.extend(self.y_min.to_le_bytes());
        data.extend(self.x_max.to_le_bytes());
        data.extend(self.y_max.to_le_bytes());
        data
    }
}

impl LineOrder {
    pub fn serialise(&self) -> Vec<u8> {
        let mut data: Vec<u8> = "lineOrder".as_bytes().to_vec();
        data.push(0);
        data.extend("lineOrder".bytes());
        data.push(0);
        data.extend(1_i32.to_le_bytes());
        let value = match self {
            Self::INCREASING_Y => 0_u8.to_le_bytes(),
            Self::DECREASING_Y => 1_u8.to_le_bytes(),
            Self::RANDOM_Y => 2_u8.to_le_bytes(),
        };
        data.extend(value);
        data
    }
}

impl pixelAspectRatio {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn serialise(&self) -> Vec<u8> {
        let mut data = "pixelAspectRatio".as_bytes().to_vec();
        data.push(0);
        data.extend("float".bytes());
        data.push(0);
        data.extend(4_i32.to_le_bytes());
        data.extend(self.value.to_le_bytes());
        data
    }
}

impl screenWindowCenter {
    pub fn new(value_x: f32, value_y: f32) -> Self {
        Self { value_x, value_y }
    }

    pub fn serialise(&self) -> Vec<u8> {
        let mut data = "screenWindowCenter".as_bytes().to_vec();
        data.push(0);
        data.extend("v2f".as_bytes());
        data.push(0);
        data.extend(8_i32.to_le_bytes());
        data.extend(self.value_x.to_le_bytes());
        data.extend(self.value_y.to_le_bytes());
        data
    }
}

pub struct screenWindowWidth {
    value: f32,
}

impl screenWindowWidth {
    pub fn new(value: f32) -> Self {
        Self { value }
    }

    pub fn serialise(&self) -> Vec<u8> {
        let mut data = "screenWindowWidth".as_bytes().to_vec();
        data.push(0);
        data.extend("float".bytes());
        data.push(0);
        data.extend(4_i32.to_le_bytes());
        data.extend(self.value.to_le_bytes());
        data
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_no_compresson() {
        let comp = Compression::No;
        let expected = {
            let mut temp = Vec::new();
            temp.extend("compression".bytes());
            temp.push(0);
            temp.extend("compression".bytes());
            temp.push(0);
            temp.extend([0x01, 0x00, 0x00, 0x00]);
            temp.push(0);
            temp
        };
        assert_eq!(comp.serialise(), expected);
    }

    #[test]
    fn test_data_window() {
        let datawin = DataWindow::new(4, 3);
        let expected = {
            let mut expected = Vec::new();
            expected.extend("dataWindow".as_bytes());
            expected.push(0);
            expected.extend("box2i".as_bytes());
            expected.push(0);
            expected.extend(16_i32.to_le_bytes());
            expected.extend(0_i32.to_le_bytes());
            expected.extend(0_i32.to_le_bytes());
            expected.extend(3_i32.to_le_bytes());
            expected.extend(2_i32.to_le_bytes());
            expected
        };
        assert_eq!(datawin.serialize(), expected);
        assert_eq!(datawin.get_res_x(), 4);
        assert_eq!(datawin.get_res_y(), 3);
    }
}
