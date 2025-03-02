use ddd::ValueObject;

#[derive(ValueObject, Debug)]
pub struct Category {
    #[field]
    name: String,
}
