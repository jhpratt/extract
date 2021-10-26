/// Returns the extracted value from the provided patterns.
///
/// Like a `match` expression, a pattern can be optionally followed by an `if` guard that has access
/// to names bound by the pattern.
///
/// ```
/// # use extract::extract;
/// let array = [1, 2, 3];
/// let first = extract!(array, [el, ..] => el);
/// assert_eq!(first, Some(1));
/// ```
#[macro_export]
macro_rules! extract {
    ($expression:expr, $(
        $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? => $output:expr
    ),+ $(,)?) => {
        match $expression {
            $($( $pattern )|+ $( if $guard )? => Some($output),)+
            _ => None
        }
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn array() {
        let array = &[1, 2, 3, 4, 5][..];

        let first = extract!(*array, [el, ..] => el);
        assert_eq!(first, Some(1));

        let second = extract!(*array, [_, el, ..] => el);
        assert_eq!(second, Some(2));

        let third = extract!(*array, [_, _, el, ..] => el);
        assert_eq!(third, Some(3));

        let multi_pattern = extract!(*array,
            [el] => el,
            [_, el] => el,
            [_, _, el] => el,
            [_, _, _, el] => el,
            [_, _, _, _, el] => el,
            [_, _, _, _, _, el] => el,
        );
        assert_eq!(multi_pattern, Some(5));

        let sixth = extract!(*array, [_, _, _, _, _, el] => el);
        assert_eq!(sixth, None);
    }
}
