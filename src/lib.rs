#![feature(min_specialization)]

struct Const<const N: usize>;

trait UnrollImpl {
    fn unroll<F: FnMut(usize)>(f: F);
}

impl<const N: usize> UnrollImpl for Const<N> {
    default fn unroll<F: FnMut(usize)>(_: F) {
        unimplemented!();
    }
}

macro_rules! unroll_impl {
    ($n:expr) => {
        impl UnrollImpl for Const<$n> {
            #[inline(always)]
            fn unroll<F: FnMut(usize)>(mut f: F) {
                seq_macro::seq!(N in 0..$n{ f(N); });
            }
        }
    };
}

impl UnrollImpl for Const<0> {
    fn unroll<F: FnMut(usize)>(_: F) {}
}
seq_macro::seq!(N in 1.. =256 {unroll_impl!(N);});

/// Call the function `f` `N` times. This is guaranteed to get unrolled.
/// Values of `N` up to `256` are supported.
#[inline(always)]
pub fn unroll<const N: usize, F: FnMut(usize)>(f: F) {
    Const::<N>::unroll(f);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unroll() {
        let mut a = 0;
        unroll::<24, _>(|i| a += i);
        assert_eq!(a, 12 * 23);
    }
}
