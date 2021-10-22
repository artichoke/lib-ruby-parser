use super::TokenRewriter;

impl std::fmt::Debug for TokenRewriter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TokenRewriter").finish()
    }
}
