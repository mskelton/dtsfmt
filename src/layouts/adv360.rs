use super::KeyboardLayout;

pub fn get_layout() -> KeyboardLayout {
    #[rustfmt::skip]
    let bindings = vec![
      0, 0, 0, 0, 0, 0, 6,                      0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 6,                      0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 0, 1,     0, 0,  0, 1,     0, 0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 0, 3,           0,  3,           0, 0, 0, 0, 0, 0,
      0, 0, 0, 0, 2,        0, 0, 0,  0, 0, 2,        0, 0, 0, 0, 0,
    ];

    let breakpoints = vec![13, 27, 45, 59];

    return KeyboardLayout {
        bindings,
        breakpoints,
    };
}
