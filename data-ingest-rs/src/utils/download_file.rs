use color_eyre::eyre::{Context, ContextCompat};
use std::fs::DirEntry;
use tokio::{fs, io::AsyncWriteExt};

use crate::extensions::StringExt;
type Error = color_eyre::Report;

pub fn clean_downloads() -> color_eyre::Result<()> {
    let dir: Vec<DirEntry> = std::fs::read_dir("./")
        .wrap_err("could not read directory")?
        .collect::<Result<_, _>>()?;

    for os_file in dir {
        let file = os_file.file_name().into_string().unwrap();
        let file_format = file.file_type();

        match (
            file_format.is_csv(),
            file_format.is_parquet(),
            file_format.is_unknown(),
        ) {
            (_, _, true) => Ok(()),
            _ => std::fs::remove_file(file.clone()),
        }
        .wrap_err(format!("could not delete file {file}"))?;
    }

    Ok(())
}

pub async fn download_file(url: String) -> color_eyre::Result<String, self::Error> {
    // TODO: concurrency
    // TODO: multiple file download at the same time (parallelism)

    let result = reqwest::get(url.clone())
        .await
        .wrap_err(format!("Could not get {url}"))?;

    let file_name = if result.status() == reqwest::StatusCode::OK {
        let org_file_name = url
            .split('/')
            .last()
            .wrap_err(format!("could get the file name {url}"))?;

        let file: Vec<String> = org_file_name.split('.').map(|s| s.to_string()).collect();

        let file_name = file.first().wrap_err(format!(
            "could not get the file name of the {org_file_name}"
        ))?;

        let ext = file.last().wrap_err(format!(
            "could not get the extension of the {org_file_name}"
        ))?;

        let timestamp = get_timestamp()?;

        let file_name = format!("{file_name}-{timestamp}.{ext}");
        write_to_file(&result.bytes().await?, file_name.clone()).await?;

        Ok(file_name)
    } else {
        Err(color_eyre::eyre::eyre!("Could not download the file {url}"))
    };

    file_name
}

fn get_timestamp() -> color_eyre::Result<String> {
    let duration = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .wrap_err("Could not get the timestamp")?;

    Ok(duration.as_millis().to_string())
}

async fn write_to_file(src: &[u8], file_name: String) -> color_eyre::Result<()> {
    let mut file = fs::File::create(file_name.clone())
        .await
        .wrap_err(format!("Could not create a file {file_name}"))?;

    file.write_all(src)
        .await
        .wrap_err(format!("Could not write to {file_name}"))
}
