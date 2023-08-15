use crate::funcs;
use crate::versionfield;
use crate::prelude::*;

pub struct Header {
    parting: versionfield::Parting,
    /// attributes are broken down to parts
    /// single part files only have one part.
    parts: Vec<HashMap<String, (String, u32, Vec<u8>)>>,
    /// attrs are stored in hashmaps. The order might
    /// be important to exr.
    attr_order: Vec<Vec<String>>,
    leftover_bytes: Vec<u8>,
}

impl Header {
    pub fn deserialize(data: &mut Vec<u8>, parting: &versionfield::Parting) -> Self {
        let mut parts = Vec::new();
        let mut attr_order = Vec::new();
        'partsloop: loop {
            let mut part_attrs: HashMap<String, (String, u32, Vec<u8>)> = HashMap::new();
            let mut part_attr_order = Vec::new();
            loop {
                let attrname = String::from_utf8(
                    funcs::get_bytes_until_null(data)
                ).unwrap();
                data.drain(..1);
                let attrtype = String::from_utf8(
                    funcs::get_bytes_until_null(data)
                ).unwrap();
                data.drain(..1);
                let attrlen = funcs::get_sized_int_4bytes(data);
                let attr_bytes = data.drain(..attrlen as usize).collect::<Vec<u8>>();
                println!(":: attr name: {}, type: {}, len: {}", &attrname, &attrtype, attrlen);
                part_attrs.insert(attrname.clone(), (attrtype, attrlen, attr_bytes));
                part_attr_order.push(attrname);
                if data[0] == 0 {
                    data.drain(..1);
                    break;
                };
            };
            parts.push(part_attrs);
            attr_order.push(part_attr_order);
            match *parting {
                versionfield::Parting::Singlepart => break 'partsloop,
                versionfield::Parting::Multipart => {
                    if data[0] == 0 {
                        data.drain(..1);
                        break 'partsloop;
                    }
                },
            }
        };
        Self {
            parting: parting.clone(),
            parts,
            attr_order,
            leftover_bytes: data.drain(..).collect::<Vec<u8>>(),
        }
    }

    fn serialize_attrs(&self) -> Vec<u8> {
        println!("\n::serializing attribs.");
        let mut data = Vec::new();
        for (part_idx, part) in self.parts.iter().enumerate() {
            for attrname in &self.attr_order[part_idx] {
                let (attrtype, attrlen, attrdata) = part.get(attrname).unwrap();
                data.extend(attrname.bytes());
                data.push(0);
                data.extend(attrtype.bytes());
                data.push(0);
                data.extend(attrlen.to_le_bytes());
                data.extend(attrdata);
            }
            data.push(0);
        }
        if self.parting == versionfield::Parting::Multipart {
            data.push(0);
        }
        data
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.extend(self.serialize_attrs());
        dbg!(&data.len());
        data.extend(self.leftover_bytes.clone());
        data
    }
}
