pub mod auth;
pub mod instance;
pub mod file;
pub mod backup;
pub mod shop;
pub mod games;

use super::SimpfunClient;

pub struct UserClient<'a> {
    pub(crate) inner: &'a SimpfunClient,
}

impl<'a> UserClient<'a> {
    pub(crate) fn new(inner: &'a SimpfunClient) -> Self {
        Self { inner }
    }
}