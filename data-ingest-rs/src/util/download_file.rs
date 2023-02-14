use tokio::{fs, io::AsyncWriteExt};

use color_eyre::eyre::Context;

pub async fn download_file(url: String, file_name: String) -> color_eyre::Result<()> {
    // TODO: concurrency
    // TODO: multiple file download at the same time (parallelism)

    let result = reqwest::get(url.clone())
        .await
        .wrap_err(format!("Could not get {url}"))?;

    if result.status() == reqwest::StatusCode::OK {
        write_to_file(&result.bytes().await?, file_name).await?;
    }

    Ok(())
}

async fn write_to_file(src: &[u8], file_name: String) -> color_eyre::Result<()> {
    let mut file = fs::File::create(file_name.clone())
        .await
        .wrap_err(format!("Could not create a file {file_name}"))?;

    file.write_all(src)
        .await
        .wrap_err(format!("Could not write to {file_name}"))
}
