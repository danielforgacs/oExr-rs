use crate::prelude::*;

pub enum ChannelType {
    /// 2 bytes
    Half(Vec<f16>),
    /// 4 bytes
    FLoat(Vec<f32>),
}

pub struct Channel {
    name: String,
    pixel_values: ChannelType,
}

impl Channel {
    pub fn new(name: impl ToString, values: ChannelType) -> Self {
        Self {
            name: name.to_string(),
            pixel_values: values,
        }
    }

    pub fn serialize(&self, res_x: usize, res_y: usize) -> Vec<Vec<u8>> {
        let mut data: Vec<Vec<u8>> = Vec::new();
        dbg!(&res_y);
        dbg!(&res_x);
        match &self.pixel_values {
            ChannelType::Half(values) => {
                dbg!(&values.len());
                for y in 0..res_y {
                    let mut line = vec![];
                    for x in 0..res_x {
                        let index = (x+1)*(y+1)-1;
                        print!("{}x{} - {}, ", x, y, index);
                        line.extend(values[index as usize].to_le_bytes());
                    }
                    println!();
                    data.push(line);
                }
            },
            ChannelType::FLoat(values) => {

            },

        }
        // println!();
        // for yy in &data {
        //     for xx in yy {
        //         print!("{}, ", xx);
        //     }
        //     println!();
        // }
        data
    }
}
