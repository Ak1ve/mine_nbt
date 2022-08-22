// TODO a nice proc macro that does all this magic LMAO
use crate::tag::Tag;


impl Into<Tag> for i32 {
    fn into(self) -> Tag {
        Tag::Int(i32)
    }
}


