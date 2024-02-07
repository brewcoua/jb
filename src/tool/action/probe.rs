use crate::tool::Tool;
use crate::api::deserial::Download;

pub trait Probe {
    /// Finds a compatible release for the tool.
    ///
    /// # Errors
    /// This function will return an error if the release cannot be found.
    ///
    /// # Panics
    /// This function will panic if the tool is not valid.
    fn sync(&mut self) -> anyhow::Result<Download>;
}

impl Probe for Tool {
    fn sync(&mut self) -> anyhow::Result<Download> {
        let result = crate::api::fetch::release(self)?;

        self.version = Some(result.tool.version.unwrap());
        self.build = Some(result.tool.build.unwrap());
        self.release = Some(result.tool.release.unwrap());

        Ok(result.download)
    }
}