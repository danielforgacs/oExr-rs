use crate::funcs;
use crate::versionfield;
use crate::attrib;
use crate::prelude::*;

pub struct Header {
    parting: versionfield::Parting,
    /// attributes are broken down to parts
    /// single part files only have one part.
    parts: Vec<HashMap<String, attrib::Attribute>>,
    /// attrs are stored in hashmaps. The order might
    /// be important to exr.
    attr_order: Vec<Vec<String>>,
    leftover_bytes: Vec<u8>,
}

impl Header {
    pub fn deserialize(data: &mut Vec<u8>, parting: &versionfield::Parting) -> Self {
        //! this func takes care of findinf the pieces of data in the header.
        //! a part header is a series of attributes. An attribute is
        //! a series of fields. the rules are:
        //!
        //! ````
        //! attrib name: text
        //! null byte
        //! attrib type: text
        //! null byte
        //! value length: 4 bytes little endian u32
        //! value
        //! ````
        let mut parts = Vec::new();
        let mut attr_order = Vec::new();
        print!(":: header attributes: [");
        'partsloop: loop {
            let mut part_attrs: HashMap<String, attrib::Attribute> = HashMap::new();
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
                let attr = attrib::Attribute::new(attrname.clone(), attrtype, attrlen, attr_bytes);
                print!("{}, ", &attrname);
                part_attrs.insert(attrname.clone(), attr);
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
        println!("]");
        Self {
            parting: parting.clone(),
            parts,
            attr_order,
            leftover_bytes: data.drain(..).collect::<Vec<u8>>(),
        }
    }

    fn serialize_attrs(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for (part_idx, part) in self.parts.iter().enumerate() {
            for attrname in &self.attr_order[part_idx] {
                let attr = part.get(attrname).unwrap();
                data.extend(attr.deserialize());
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
        data.extend(self.leftover_bytes.clone());
        data
    }
}
