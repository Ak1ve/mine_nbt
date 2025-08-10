use crate::tag::Tag;
use std::collections::HashMap;
use proc_macro::TokenStream;
use quote::quote;

pub trait TagProxy<'a> {
    fn from_tag(tag: &'a mut Tag) -> Self;
    fn as_tag(&mut self) -> &mut Tag;
}

// #[proc_macro_derive(TagProxy)]
// pub fn derive_proxy(item: TokenStream) -> TokenStream {
//     quote!()
// }


macro_rules! impl_proxy {
    ($i: ident) => {
        impl<'a> TagProxy<'a> for $i <'a> {
            fn from_tag(tag: &'a mut Tag) -> Self {
                Self {
                    tag
                }
            }
            fn as_tag(&mut self) -> &mut Tag {
                &mut self.tag
            }
        }
    };
}
pub struct Level {
    tag: Tag
}


impl Level {
    pub fn from_tag(tag: Tag) -> Self {
        Self {
            tag
        }
    }

    pub fn as_tag(&mut self) -> &mut Tag {
        &mut self.tag
    }

    pub fn data(&mut self) -> Option<Data> {
        Some(Data::from_tag(self.as_tag().compound()?.get_mut("Data")?))
    }
}

pub struct Data<'a> {
    tag: &'a mut Tag
}


impl_proxy!(Data);

impl<'a> Data<'a> {
    pub fn allow_commands(&mut self) -> Option<&mut u8> {
        Some(self.tag.compound()?.get_mut("allowCommands")?.byte()?)
    }

    pub fn raining(&mut self) -> Option<&mut u8> {
        Some(self.tag.compound()?.get_mut("raining")?.byte()?)
    }

    pub fn server_brands(&mut self) -> Option<&mut Vec<Tag>> {
        Some(self.tag.compound()?.get_mut("ServerBrands")?.list()?)

    }
}

// USE https://github.com/pest-parser/pest/blob/master/derive/src/lib.rs