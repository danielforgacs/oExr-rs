pub struct DataWin {
    x_min: u32,
    y_min: u32,
    x_max: u32,
    y_max: u32,
}

impl From<Vec<u8>> for DataWin {
    fn from(mut value: Vec<u8>) -> Self {
        let x_min = u32::from_le_bytes(value.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
        let y_min = u32::from_le_bytes(value.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
        let x_max = u32::from_le_bytes(value.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
        let y_max = u32::from_le_bytes(value.drain(..4).collect::<Vec<u8>>().try_into().unwrap());
        Self { x_min, y_min, x_max, y_max }
    }
}

impl DataWin {
    pub fn serialize(&self) -> Vec<u8> {
        let mut data: Vec<u8> = "dataWindow".bytes().collect();
        data.push(0);
        data.extend("box2i".bytes().collect::<Vec<u8>>());
        data.push(0);
        data.extend(16_u32.to_le_bytes());
        data.extend(self.x_min.to_le_bytes());
        data.extend(self.y_min.to_le_bytes());
        data.extend(self.x_max.to_le_bytes());
        data.extend(self.y_max.to_le_bytes());
        data
    }

    pub fn x_min(&self) -> u32 { self.x_min }
    pub fn y_min(&self) -> u32 { self.y_min }
    pub fn x_max(&self) -> u32 { self.x_max }
    pub fn y_max(&self) -> u32 { self.y_max }

    pub fn get_res_x(&self) -> u32 {
            self.x_max - self.x_min + 1
    }

    pub fn get_res_y(&self) -> u32 {
            self.y_max - self.y_min + 1
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_can_deserialize_and_serialize_data_window() {
        let raw_data = vec![
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x03, 0x00, 0x00, 0x00,
            0x02, 0x00, 0x00, 0x00,
        ];
        let expected = vec![
            0x64, 0x61, 0x74, 0x61, 0x57, 0x69, 0x6E, 0x64,0x6F, 0x77,
            0x0,
            0x62, 0x6F, 0x78, 0x32, 0x69,
            0x0,
            0x10, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00,
            0x03, 0x00, 0x00, 0x00,
            0x02, 0x00, 0x00, 0x00,
        ];
        let data_win = DataWin::from(raw_data.clone());
        let data = data_win.serialize();
        assert_eq!(data, expected);
    }
}
