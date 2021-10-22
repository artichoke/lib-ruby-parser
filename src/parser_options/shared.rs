crate::use_native_or_external!(Maybe);
crate::use_native_or_external!(String);

use bumpalo::Bump;

use super::ParserOptions;

const DEFAULT_BUFFER_NAME: &str = "(eval)";

impl<'a> ParserOptions<'a> {
    fn default(bump: &'a Bump) -> Self {
        Self::new(
            String::from_str_in(DEFAULT_BUFFER_NAME, bump),
            Maybe::none(),
            Maybe::none(),
            true,
        )
    }
}
