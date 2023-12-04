#[macro_export]
macro_rules! hashmap {
    (,) => {
        std::compile_error("This macro doesn't accept only comma");
    };
    ( $($k:expr => $v:expr),* $(,)? ) => {
        {
            use ::std::collections::HashMap;
            let mut hm = HashMap::new();
            $(
                hm.insert($k, $v);
            )*
            hm
        }
    };
}
