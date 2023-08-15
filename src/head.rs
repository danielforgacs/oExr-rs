use crate::funcs;
use crate::versionfield;
use crate::prelude::*;

type AttrMap = HashMap<(usize, String), (String, u32, Vec<u8>)>;

pub struct Header {
    parting: versionfield::Parting,
    attrs: AttrMap,
    attr_order: Vec<(usize, String)>,
    leftover_bytes: Vec<u8>,
}

impl Header {
    pub fn deserialize(data: &mut Vec<u8>, parting: &versionfield::Parting) -> Self {
        let mut attrs: AttrMap = HashMap::new();
        let mut attr_order = Vec::new();
        // multi part headers have some attrs once for every header.
        // this is the index that's added to the header name in th hashmap.
        let mut part_index = 0;
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
            attr_order.push((part_index, attrname.clone()));
            attrs.insert((part_index, attrname), (attrtype, attrlen, attr_bytes));
            // single part header ends with a null
            // multi part headers end with a null and the headers
            // endwith an empty header - so there's a double 0
            // when the headers are finished.
            match parting {
                versionfield::Parting::Singlepart => {
                    if data[0] == 0 {
                        break;
                    }
                },
                versionfield::Parting::Multipart => {
                    if data[..2] == [0, 0] {
                        break;
                    }
                    if data[0] == 0 {
                        part_index += 1;
                        // removing the separator null between headers.
                        data.drain(..1);
                    }
                },
            };
        }
        // removing the null at the end of the header(s)
        data.drain(..1);
        Self {
            parting: parting.clone(),
            attrs,
            attr_order,
            leftover_bytes: data.drain(..).collect::<Vec<u8>>(),
        }
    }

    fn serialize_attrs(&self) -> Vec<u8> {
        println!("\n::serializing attribs.");
        let mut data = Vec::new();
        let mut previous_part_index = 0;
        for (part_idx, name) in &self.attr_order {
            println!("part index: {}", part_idx);
            let (attrtype, attrlen, attrdata) = self.attrs.get(&(*part_idx, name.to_string())).unwrap();
            let attrname = match self.parting {
                versionfield::Parting::Singlepart => name,
                versionfield::Parting::Multipart => {
                    if *part_idx != previous_part_index {
                        previous_part_index = *part_idx;
                        data.push(0);
                    }
                    name
                },
            };
            data.extend(attrname.bytes());
            data.push(0);
            data.extend(attrtype.bytes());
            data.push(0);
            data.extend(attrlen.to_le_bytes());
            data.extend(attrdata);
        }
        data.push(0);
        data
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.extend(self.serialize_attrs());
        data.extend(self.leftover_bytes.clone());
        data
    }
}
