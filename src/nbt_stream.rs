use std::fs::File;
use std::io;
use std::io::{Bytes, Read, Write};
use std::iter::Fuse;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};
use iter_read::IterRead;
use paste::paste;


pub struct NBTRead {
    stream: GzDecoder<IterRead<io::Result<u8>, Fuse<Bytes<File>>>>
}

impl NBTRead {
    pub fn new(path: &str) -> io::Result<Self> {
        Ok(Self {
            stream: GzDecoder::new(IterRead::new(File::open(path)?.bytes().fuse()))
        })
    }

    pub fn consume_bytes(&mut self, n: usize) -> io::Result<Vec<u8>> {
        let mut buffer = vec![0u8; n];
        self.stream.read_exact(&mut buffer)?;
        Ok(buffer)
    }

    pub fn consume<T>(&mut self) -> io::Result<T> where
        T: Consumable
    {
        T::consume(self)
    }
}


pub struct NBTWrite<W> where W: Write {
    stream: GzEncoder<W>
}


impl<W> NBTWrite<W> where W: Write {

}

pub trait Consumable {
    fn consume(stream: &mut NBTRead) -> io::Result<Self> where
        Self: Sized;
}


macro_rules! impl_consumable {
    ($t: ident) => {
        paste! {
            impl Consumable for $t {
                fn consume(stream: &mut NBTRead) -> io::Result<Self>
                {
                    stream.stream.[<read_ $t>]::<BigEndian>()
                }
            }
    }
    };
}

impl Consumable for i8 {
    fn consume(stream: &mut NBTRead) -> io::Result<Self> where Self: Sized {
        stream.stream.read_i8()
    }
}

impl Consumable for u8 {
    fn consume(stream: &mut NBTRead) -> io::Result<Self> where Self: Sized {
        stream.stream.read_u8()
    }
}

impl Consumable for String {
    fn consume(stream: &mut NBTRead) -> io::Result<Self> where Self: Sized {
        let length = u16::consume(stream)? as usize;
        let bytes = stream.consume_bytes(length)?;
        Ok(String::from_utf8(bytes).unwrap())
    }
}

impl_consumable!(i16);
impl_consumable!(i32);
impl_consumable!(i64);
impl_consumable!(i128);
impl_consumable!(u16);
impl_consumable!(u32);
impl_consumable!(u64);
impl_consumable!(u128);
impl_consumable!(f32);
impl_consumable!(f64);

pub trait ToStream {
    fn to_stream<W: Write>(self, stream: &mut NBTWrite<W>) -> io::Result<()>;
}

impl ToStream for i8 {
    fn to_stream<W: Write>(self, stream: &mut NBTWrite<W>) -> io::Result<()> {
        stream.stream.write_i8(self)
    }
}

impl ToStream for u8 {
    fn to_stream<W: Write>(self, stream: &mut NBTWrite<W>) -> io::Result<()> {
        stream.stream.write_u8(self)
    }
}

impl ToStream for String {
    fn to_stream<W: Write>(self, stream: &mut NBTWrite<W>) -> io::Result<()> {
        (self.len() as u16).to_stream(stream)?;
        stream.stream.write_all(self.as_bytes())
    }
}

macro_rules! impl_to_stream {
    ($t: ident) => {
        paste! {
            impl ToStream for $t {
                fn to_stream<W: Write>(self, stream: &mut NBTWrite<W>) -> io::Result<()> {
                    stream.stream.[<write_ $t>]::<BigEndian>(self)
                }
            }
        }
    };
}

impl_to_stream!(i16);
impl_to_stream!(i32);
impl_to_stream!(i64);
impl_to_stream!(i128);
impl_to_stream!(u16);
impl_to_stream!(u32);
impl_to_stream!(u64);
impl_to_stream!(u128);
impl_to_stream!(f32);
impl_to_stream!(f64);