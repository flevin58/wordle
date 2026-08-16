#[macro_export]
macro_rules! get_read_context {
    () => {
       crate::game::CONTEXT.get().unwrap().read().unwrap()
    };
}
