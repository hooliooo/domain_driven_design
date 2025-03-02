#[macro_export]
macro_rules! generate_entity_equality_hash {
    (
        $aggregate_name: ident
    ) => {
        impl PartialEq for $aggregate_name {
            fn eq(&self, other: &Self) -> bool {
                self.id() == other.id()
            }
        }

        impl Eq for $aggregate_name {}

        impl Hash for $aggregate_name {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.id.hash(state);
            }
        }
    };
}
