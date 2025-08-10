use std::collections::HashMap;
use std::io;
use crate::proxy::{Level, TagProxy};
use crate::nbt_stream::{Consumable, NBTRead};
use crate::tag::Tag;

mod nbt_stream;
mod tag;
mod proxy;

fn main_tag() -> Tag {
    Tag::from_stream(&mut NBTRead::new("level.dat").unwrap()).unwrap()
}
fn real_main() -> Option<()> {
    let mut level = Level::from_tag(main_tag());
    *level.data()?.server_brands()? = vec![Tag::String("Dog".to_string())];
    println!("{:?}", level.as_tag());
    Some(())
}

#[cfg(test)]
mod tests {
    use std::io;
    use crate::{NBTRead, real_main, tag};
    use crate::tag::Tag;

    #[test]
    pub fn main() {
        let _ = real_main();
    }
}