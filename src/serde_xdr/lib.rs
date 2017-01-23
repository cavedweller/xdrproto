#![cfg_attr(test, feature(custom_attribute, custom_derive, plugin))]

extern crate serde;
extern crate byteorder;

pub mod ser;
pub mod error;

use serde::Serialize;
pub use self::error::{EncoderError, EncoderResult};
//pub use self::xdr_values::{XdrValue};

pub use self::ser::{
    Serializer,
    //to_writer,
    //to_vec,
    //value_to_writer,
    //value_to_vec,
};

//pub fn serialize<T>(value: &T, size_limit: SizeLimit) -> SerializeResult<Vec<u8>>
//    where T: serde::Serialize,
//{
//    // Since we are putting values directly into a vector, we can do size
//    // computation out here and pre-allocate a buffer of *exactly*
//    // the right size.
//    let mut writer = match size_limit {
//        SizeLimit::Bounded(size_limit) => {
//            let actual_size = match serialized_size_bounded(value, size_limit) {
//                Some(actual_size) => actual_size,
//                None => { return Err(SerializeError::SizeLimit); }
//            };
//            Vec::with_capacity(actual_size as usize)
//        }
//        SizeLimit::Infinite => Vec::new()
//    };
//
//    try!(serialize_into(&mut writer, value, SizeLimit::Infinite));
//    Ok(writer)
//}


pub fn to_bytes<T>(value: &T) -> EncoderResult<Vec<u8>>
where T: Serialize
{
    let mut writer = Vec::with_capacity(128);
    {
        let mut ser = Serializer::new(&mut writer);
        try!(value.serialize(&mut ser));
    }
//  Ok(ser.into_inner())
    Ok(writer)
}
