use num_traits::PrimInt;
use std::fmt::Debug;
pub struct Luhn {
    code: String,
}

impl Luhn {
    pub fn is_valid(&self) -> bool {
        self.code
            .chars()
            .rev()
            .filter(|c| !c.is_whitespace())
            .try_fold((0, 0), |(sum, count), val| {
                val.to_digit(10)
                    .map(|num| if count % 2 == 1 { num * 2 } else { num })
                    .map(|num| if num > 9 { num - 9 } else { num })
                    .map(|num| (sum + num, count + 1))
            })
            .map_or(false, |(sum, count)| sum % 10 == 0 && count > 1)
    }
}

/// Here is the example of how the From trait could be implemented
/// for the &str type. Naturally, you can implement this trait
/// by hand for the every other type presented in the test suite,
/// but your solution will fail if a new type is presented.
/// Perhaps there exists a better solution for this problem?
impl<'a> From<&'a str> for Luhn {
    fn from(input: &'a str) -> Self {
        // 'input' is a borrowed string (&str), can be used directly
        Self {
            code: input.to_string(),
        }
    }
}

impl From<String> for Luhn {
    fn from(input: String) -> Self {
        Self { code: input }
    }
}
macro_rules! impl_trait_for_unsigned {
    ($($t:ty),*) => {
        $(
            impl From<$t> for Luhn {
                fn from(input: $t) -> Self {
                    Self { code: input.to_string()}
                }
            }
        )*
    }
}
impl_trait_for_unsigned!(u8, u16, u32, u64, usize);
