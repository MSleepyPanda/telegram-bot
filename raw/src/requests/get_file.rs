use std::borrow::Cow;

use requests::*;
use types::*;

/// Use this method to get basic info about a file and prepare it for downloading.
/// For the moment, bots can download files of up to 20MB in size.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetFile<'s> {
    file_id: Cow<'s, str>,
}

impl<'s> Request for GetFile<'s> {
    type Response = IdResponse<File>;

    fn name(&self) -> &'static str {
        "getFile"
    }
}

impl<'s> GetFile<'s> {
    pub fn new<F>(file_id: F) -> Self where F: Into<Cow<'s, str>> {
        Self {
            file_id: file_id.into()
        }
    }
}
