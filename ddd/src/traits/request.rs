use std::hash::Hash;

pub trait Request<'a> {
    type RequestId: Eq + PartialEq + Hash + Clone;
    /// The unique identifier of the Request
    fn request_id(&'a self) -> &'a Self::RequestId;

    type IssuerId: Eq + PartialEq + Hash + Clone;
    /// The identifier of the issuer of the command
    fn issuer_id(&'a self) -> &'a Self::IssuerId;
}
