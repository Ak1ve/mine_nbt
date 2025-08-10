// parsing for https://minecraft.fandom.com/wiki/NBT_format#Conversion_to_SNBT

use crate::{Consumable, NBTRead};
use std::any::TypeId;
use std::collections::HashMap;
use std::hash::Hash;
use std::io;
use std::mem::transmute;

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
    LongArray(Vec<i64>),
}

fn consume_array<T: Consumable>(stream: &mut NBTRead) -> io::Result<Vec<T>> {
    let mut array: Vec<T> = Vec::with_capacity(stream.consume::<i32>()? as usize);
    for _ in 0..array.capacity() {
        array.push(T::consume(stream)?);
    }
    Ok(array)
}

fn tag_by_id(
    stream: &mut NBTRead,
    id: u8,
    include_name: bool,
) -> io::Result<(Option<String>, Tag)> {
    let name: Option<String> = if include_name && id != 0 {
        Some(stream.consume::<String>()?)
    } else {
        None
    };
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
        _ => unreachable!(),
    };

    Ok((name, tag))
}

fn parse_nbt(stream: &mut NBTRead, include_name: bool) -> io::Result<(Option<String>, Tag)> {
    let id: u8 = stream.consume()?;
    tag_by_id(stream, id, include_name)
}

macro_rules! conversion {
    ($name: ident, $tag: expr, $typ: ty) => {
        pub fn $name(&mut self) -> Option<&mut $typ> {
            use paste::paste;
            match self {
                paste!($tag(x)) => Some(x),
                _ => None,
            }
        }
    };
}

impl Tag {
    pub fn from_stream(stream: &mut NBTRead) -> io::Result<Tag> {
        let (_, tag) = parse_nbt(stream, true)?;
        Ok(tag)
    }

    conversion!(compound, Tag::Compound, HashMap<String, Tag>);
    conversion!(float, Tag::Float, f32);
    conversion!(double, Tag::Double, f64);
    conversion!(int, Tag::Int, i32);
    conversion!(long, Tag::Long, i64);
    conversion!(byte, Tag::Byte, u8);
    conversion!(string, Tag::String, String);
    conversion!(list, Tag::List, Vec<Tag>);

    pub fn int_array(&mut self) -> Option<&mut [i32]> {
        match self {
            Tag::IntArray(x) => Some(x.as_mut_slice()),
            _ => None,
        }
    }
}

pub trait ToTag {
    fn to_tag(self) -> Tag;
}

macro_rules! tag_impl {
    ($tag: expr, $typ: ty) => {
        impl ToTag for $typ {
            fn to_tag(self) -> Tag {
                use paste::paste;
                paste!($tag(self))
            }
        }
    };
}

tag_impl!(Tag::Byte, u8);
tag_impl!(Tag::Short, i16);
tag_impl!(Tag::Int, i32);
tag_impl!(Tag::Long, i64);
tag_impl!(Tag::Float, f32);
tag_impl!(Tag::Double, f64);
tag_impl!(Tag::String, String);

impl<'a> ToTag for &'a str {
    fn to_tag(self) -> Tag {
        Tag::String(self.to_string())
    }
}

impl<T> ToTag for Vec<T>
where
    T: ToTag + 'static,
{
    fn to_tag(self) -> Tag {
        let t = TypeId::of::<T>();
        if t == TypeId::of::<u8>() {
            return Tag::ByteArray(unsafe { transmute(self) });
        }
        if t == TypeId::of::<i32>() {
            return Tag::IntArray(unsafe { transmute(self) });
        }
        if t == TypeId::of::<i64>() {
            return Tag::LongArray(unsafe { transmute(self) });
        }
        let mut vec = Vec::with_capacity(self.len());
        for v in self {
            vec.push(v.to_tag());
        }
        Tag::List(vec)
    }
}

impl<T> ToTag for HashMap<String, T> where T: ToTag {
    fn to_tag(self) -> Tag {
        Tag::Compound(self.into_iter().map(|(k, v)| (k, v.to_tag())).collect())
    }
}

impl<'a, T> ToTag for HashMap<&'a str, T> where T: ToTag {
    fn to_tag(self) -> Tag {
        Tag::Compound(self.into_iter().map(|(k, v)| (k.to_string(), v.to_tag())).collect())
    }
}


// Byte(u8),
// Short(i16),
// Int(i32),
// Long(i64),
// Float(f32),
// Double(f64),
// ByteArray(Vec<u8>),
// String(String),
// List(Vec<Tag>),
// Compound(HashMap<String, Tag>),
// IntArray(Vec<i32>),
// LongArray(Vec<i64>)
