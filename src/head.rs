use crate::attrib;
use crate::datawin;
use crate::funcs;
use crate::prelude::*;
use crate::vfield;

pub struct Header {
    parting: vfield::Parting,
    /// attributes are broken down to parts
    /// single part files only have one part.
    parts: Vec<HashMap<String, attrib::Attribute>>,
    /// attrs are stored in hashmaps. The order might
    /// be important to exr.
    attr_order: Vec<Vec<String>>,
    data_window: Vec<datawin::DataWin>,
}

impl Header {
    pub fn deserialize(data: &mut Vec<u8>, parting: &vfield::Parting) -> Self {
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
        let mut data_window = Vec::new();
        'partsloop: loop {
            let mut part_attrs: HashMap<String, attrib::Attribute> = HashMap::new();
            let mut part_attr_order = Vec::new();
            loop {
                let attrname = String::from_utf8(funcs::get_bytes_until_null(data)).unwrap();
                data.drain(..1);
                let attrtype = String::from_utf8(funcs::get_bytes_until_null(data)).unwrap();
                data.drain(..1);
                let attrlen = funcs::get_sized_int_4bytes(data);
                let attr_bytes = data.drain(..attrlen as usize).collect::<Vec<u8>>();
                if attrname == "dataWindow" {
                    let data_win = datawin::DataWin::from(attr_bytes);
                    data_window.push(data_win);
                    part_attr_order.push(attrname.clone());
                } else {
                    let attr =
                        attrib::Attribute::new(attrname.clone(), attrtype, attrlen, attr_bytes);
                    part_attrs.insert(attrname.clone(), attr);
                    part_attr_order.push(attrname.clone());
                };
                if data[0] == 0 {
                    data.drain(..1);
                    break;
                };
            }
            parts.push(part_attrs);
            attr_order.push(part_attr_order);
            match *parting {
                vfield::Parting::Singlepart => break 'partsloop,
                vfield::Parting::Multipart => {
                    if data[0] == 0 {
                        data.drain(..1);
                        break 'partsloop;
                    }
                }
            }
        }
        Self {
            parting: parting.clone(),
            parts,
            attr_order,
            data_window,
        }
    }

    pub fn serialize(&self) -> Vec<u8> {
        let mut data = Vec::new();
        for (part_idx, part) in self.parts.iter().enumerate() {
            for attrname in &self.attr_order[part_idx] {
                if attrname == "dataWindow" {
                    let data_win = &self.data_window[part_idx];
                    data.extend(data_win.serialize());
                } else {
                    let attr = part.get(attrname).unwrap();
                    data.extend(attr.deserialize());
                }
            }
            data.push(0);
        }
        if self.parting == vfield::Parting::Multipart {
            data.push(0);
        }
        data
    }

    pub fn get_res_y(&self) -> u32 {
        //
        // HARDCODED INDEX
        //
        self.data_window[0].get_res_y() + 1
    }
}
