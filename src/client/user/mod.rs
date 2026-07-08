pub mod auth;
pub mod backup;
pub mod file;
pub mod games;
pub mod instance;
pub mod shop;

use super::SimpfunClient;

pub struct UserClient<'a> {
    pub(crate) inner: &'a SimpfunClient,
}

impl<'a> UserClient<'a> {
    pub(crate) fn new(inner: &'a SimpfunClient) -> Self {
        Self { inner }
    }
}
