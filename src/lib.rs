use std::collections::HashMap;
use std::io;
use crate::nbt_stream::{Consumable, NBTRead};

mod nbt_stream;
mod tag;
mod proxy;
mod level;


#[cfg(test)]
mod tests {
    use std::io;
    use crate::NBTRead;
    use crate::tag::Tag;

    #[test]
    pub fn main() -> io::Result<()> {
        // let mut stream = NBTRead::new("level.dat")?;
        // let x = Tag::from_stream(&mut stream)?;
        // println!("{x:?}");
        Ok(())
    }
}