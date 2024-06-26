use std::{path::PathBuf, sync::Arc};

use downloader::progress::Reporter;

pub trait ComponentDownloader {
    #[allow(async_fn_in_trait)]
    async fn install<P: Reporter + 'static>(&self, progress: Option<Arc<P>>) -> anyhow::Result<()>;

    //the 'static is something to change, I don't very like it, but it's for testing purpose
    #[allow(async_fn_in_trait)]
    async fn download<P: Reporter + 'static>(
        &self,
        output_dir: &PathBuf,
        progress: Option<Arc<P>>,
    ) -> anyhow::Result<PathBuf>;

    #[allow(async_fn_in_trait)]
    async fn uncompress(file: PathBuf, new_filename: PathBuf) -> anyhow::Result<()>;
}
