use super::SimpfunClient;

pub mod image;

pub struct DevClient<'a> {
    pub(crate) inner: &'a SimpfunClient,
}

impl<'a> DevClient<'a> {
    pub(crate) fn new(inner: &'a SimpfunClient) -> Self {
        Self { inner }
    }
}