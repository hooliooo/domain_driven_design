use std::hash::Hash;

/// A **ValueObject** is an object whose equality is not defined by identity but by its attributes
///
/// Two value objects are the same if all of their attributes are equal
/// ```
/// #![cfg(feature = "derive")]
///
/// use ddd::ValueObject;
/// use ddd::traits::value_object::ValueObject;
///
/// #[derive(ddd::ValueObject, Debug)]
/// pub struct Coordinate {
///     x: i32,
///     y: i32,
/// }
///
/// impl Coordinate {
///     pub fn new(x: i32, y: i32) -> Self {
///         Self { x, y }
///     }
/// }
///
/// let a = Coordinate::new(1, 2);
/// let b = Coordinate::new(1, 2);
///
/// assert_eq!(a, b);
///
/// let c = Coordinate::new(2, 2);
/// let d = Coordinate::new(3, 3);
///
/// assert_ne!(c, d);
///
/// #[derive(ddd::ValueObject, Debug)]
/// enum Message {
///     Quit,
///     Move { x: i32, y: i32 },
///     Write(String),
///     ChangeColor(i32, i32, i32),
/// }
///
/// let a = Message::Quit;
/// let b = Message::Quit;
///
/// assert_eq!(a, b);
///
/// let a = Message::Quit;
/// let b = Message::Write("Thing".into());
///
/// assert_ne!(a, b);
///
/// let a = Message::Move { x: 2, y: 2 };
/// let b = Message::Move { x: 2, y: 2 };
///
/// assert_eq!(a, b);
///
/// let a = Message::Move { x: 2, y: 3 };
/// let b = Message::Move { x: 3, y: 4 };
///
/// assert_ne!(a, b);
///
/// let a = Message::Write("Tom".into());
/// let b = Message::Write("Tom".into());
///
/// assert_eq!(a, b);
///
/// let a = Message::Write("Tom".into());
/// let b = Message::Write("Jerry".into());
///
/// assert_ne!(a, b);
///
/// ```
pub trait ValueObject: Eq + PartialEq + Hash + Clone {}
