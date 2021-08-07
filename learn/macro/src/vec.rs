macro_rules! vec_strs {
    (
        $(
            $element:expr
        )
        , // sep, such as , ; and so on
        * // rep, * or +
    ) => {
        {
            let mut v = Vec::new();
            $(
                v.push(format!("{}", $element));
            )*
            v
        }
    };
}

#[test]
fn test() {
    let s = vec_strs![1, "a", true, 3.14159f32];
    assert_eq!(&*s, &["1", "a", "true", "3.14159"]);
}
