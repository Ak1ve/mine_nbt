use std::alloc::alloc;
use std::collections::HashMap;
use std::hash::Hash;
use std::marker::PhantomData;
// TODO a nice proc macro that does all this magic LMAO
use crate::tag::Tag;
//  https://minecraft.fandom.com/wiki/Java_Edition_level_format#level.dat_format
use serde::{Serialize, Deserialize, Serializer, Deserializer};
use paste::paste;


#[derive(Debug)]
struct Data {
    // #[from(Tag::Byte)]
    // #[name(allowCommands)]
    allow_commands: bool,
    // #[from(Tag::Double)]
    // #[name(BorderCenterX)]
    border_center_x: f64,
    border_center_z: f64
}

impl Data {
    fn new(x: &HashMap<String, Tag>) -> Option<Self> {
        Some(Self {
            allow_commands: {match x.get("allowCommands")? {
                Tag::Byte(x) => Some(*x != 0u8),
                _ => None
            }?},
            border_center_x: {match x.get("BorderCenterX")? {
                Tag::Double(x) => Some(*x),
                _ => None
            }?},
            border_center_z: {match x.get("BorderCenterX")? {
                Tag::Double(x) => Some(*x),
                _ => None
            }?}
        })
    }
}

struct Level {
    data: Data
}


macro_rules! require {
    ($map: expr, $tag: expr) => {
        {
            use paste::paste;

        }
    };
}

#[cfg(test)]
mod tests {
    use std::io;
    use crate::NBTRead;
    use crate::proxy::Data;
    use crate::tag::Tag;

    #[test]
    fn main() -> io::Result<()> {
        let mut reader = NBTRead::new("level.dat")?;
        let x = Tag::from_stream(&mut reader)?;
        let y = {
            match x {
                Tag::Compound(x) => x,
                _ => unreachable!()
            }
        };

        println!("{:?}", y);

        Ok(())
    }
}