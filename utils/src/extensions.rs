
pub trait IsNoneOr<T> {
    #[allow(clippy::wrong_self_convention)]
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool;
}

impl<T> IsNoneOr<T> for Option<T> {
    fn is_none_or(self, f: impl FnOnce(T) -> bool) -> bool {
        match self {
            None => true,
            Some(x) => f(x),
        }
    }
}

#[test]
fn is_none_or_works() {
    let x: Option<u32> = Some(2);
    assert_eq!(x.is_none_or(|x| x > 1), true);

    let x: Option<u32> = Some(0);
    assert_eq!(x.is_none_or(|x| x > 1), false);

    let x: Option<u32> = None;
    assert_eq!(x.is_none_or(|x| x > 1), true);
}
