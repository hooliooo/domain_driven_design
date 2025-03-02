use ddd::ValueObject;

#[derive(ValueObject, Debug)]
pub struct Tag {
    #[field]
    name: String,
}
