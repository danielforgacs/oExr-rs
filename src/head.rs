use crate::funcs;
use crate::versionfield;
use crate::prelude::*;

pub struct Header {
    attrs: HashMap<String, (String, u32, Vec<u8>)>,
    leftover_bytes: Vec<u8>,
}

impl Header {
    pub fn deserialize(data: &mut Vec<u8>, parting: &versionfield::Parting) -> Self {
        let mut attrs: HashMap<String, (String, u32, Vec<u8>)> = HashMap::new();
        // multi part headers have some attrs once for every header.
        // this is the index that's added to the header name in th hashmap.
        let mut part_index = 0;
        loop {
            let attrname = String::from_utf8(
                funcs::get_bytes_until_null(data)
            ).unwrap();
            let attrname = match parting {
                versionfield::Parting::Singlepart => attrname,
                versionfield::Parting::Multipart => {
                    let mut attrname = attrname;
                    attrname.push('#');
                    attrname += format!("{}", part_index).as_ref();
                    attrname
                },

            };
            data.drain(..1);
            let attrtype = String::from_utf8(
                funcs::get_bytes_until_null(data)
            ).unwrap();
            data.drain(..1);
            let attrlen = funcs::get_sized_int_4bytes(data);
            println!(":: found attr: {}, type: {}, lenght: {}", attrname, attrtype, attrlen);
            let attr_bytes = data.drain(..attrlen as usize).collect::<Vec<u8>>();
            attrs.insert(attrname, (attrtype, attrlen, attr_bytes));
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
                        data.drain(..1);
                    }
                },
            };
        }
        Self {
            attrs,
            leftover_bytes: data.drain(..).collect::<Vec<u8>>(),
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.extend(self.leftover_bytes.clone());
        data
    }
}
