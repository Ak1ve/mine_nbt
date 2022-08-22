// parsing for https://minecraft.fandom.com/wiki/NBT_format#Conversion_to_SNBT

use std::collections::HashMap;
use std::io;
use crate::{Consumable, NBTRead};


#[derive(PartialEq, Debug)]
pub enum Tag {
    END,
    Byte(u8),
    Short(i16),
    Int(i32),
    Long(i64),
    Float(f32),
    Double(f64),
    ByteArray(Vec<u8>),
    String(String),
    List(Vec<Tag>),
    Compound(HashMap<String, Tag>),
    IntArray(Vec<i32>),
    LongArray(Vec<i64>)
}

fn consume_array<T: Consumable>(stream: &mut NBTRead) -> io::Result<Vec<T>> {
    let mut array: Vec<T> = Vec::with_capacity(stream.consume::<i32>()? as usize);
    for _ in 0..array.capacity() {
        array.push(T::consume(stream)?);
    }
    Ok(array)
}

fn tag_by_id(stream: &mut NBTRead, id: u8, include_name: bool) -> io::Result<(Option<String>, Tag)> {
    let name: Option<String> = if include_name && id != 0 { Some(stream.consume::<String>()?) } else { None };
    let tag = match id {
        0 => Tag::END,
        1 => Tag::Byte(stream.consume()?),
        2 => Tag::Short(stream.consume()?),
        3 => Tag::Int(stream.consume()?),
        4 => Tag::Long(stream.consume()?),
        5 => Tag::Float(stream.consume()?),
        6 => Tag::Double(stream.consume()?),
        7 => Tag::ByteArray(consume_array(stream)?),
        8 => Tag::String(stream.consume()?),
        9 => Tag::List({
            let tag_id: u8 = stream.consume()?;
            let mut list: Vec<Tag> = Vec::with_capacity(stream.consume::<i32>()? as usize);
            for _ in 0..list.capacity() {
                let (_, tag) = tag_by_id(stream, tag_id, false)?;
                list.push(tag);
            }
            list

        }),
        10 => Tag::Compound({
            let mut map = HashMap::new();
            loop {
                let (name, tag) = parse_nbt(stream, true)?;
                if tag == Tag::END {
                    break;
                }
                map.insert(name.expect("includes name"), tag);
            }
            map
        }),
        11 => Tag::IntArray(consume_array(stream)?),
        12 => Tag::LongArray(consume_array(stream)?),
        _ => unreachable!()
    };

    Ok((name, tag))
}

fn parse_nbt(stream: &mut NBTRead, include_name: bool) -> io::Result<(Option<String>, Tag)> {
    let id: u8 = stream.consume()?;
    tag_by_id(stream, id, include_name)
}

impl Tag {
    pub fn from_stream(stream: &mut NBTRead) -> io::Result<Tag> {
        let (_, tag) = parse_nbt(stream, true)?;
        Ok(tag)
    }
}
