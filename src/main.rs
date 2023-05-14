use std::borrow::BorrowMut;
use std::env;
use std::fs;
use std::fs::File;
use std::future::IntoFuture;
use std::io::Error;
use std::io::Read;
use azure_core::error::{ErrorKind, ResultExt};
use azure_storage_blobs::blob;
use azure_storage_blobs::prelude::ClientBuilder;
use azure_storage_blobs::prelude::*;
use azure_storage::prelude::*;
use dotenv::dotenv;

#[tokio::main]
async fn main() -> azure_core::Result<() > {
    dotenv().ok();

    let account = env::var("ACCOUNT").unwrap();
    let acces_key = env::var("ACCESS_KEY").unwrap();
    let container = env::var("CONTAINER_NAME").unwrap();
    let storage_credential =
        StorageCredentials::Key(account.to_string(), acces_key.to_string());
    let blob_client = ClientBuilder::new(account.clone(), storage_credential);

    println!("{}", account);
    let args: Vec<_> = env::args().collect();
    let binding_directory = (&args[1]).to_string();
    println!("{}", &binding_directory);
    let paths = fs::read_dir(binding_directory)
        .unwrap();
    for path in paths {
        let string_path = path.unwrap().path();
        let extension = string_path
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or("");
        let path_name = string_path
            .clone()
            .into_os_string()
            .into_string()
            .unwrap();
        let file_name = string_path
            .clone()
            .file_name()
            .unwrap()
            .to_os_string()
            .into_string()
            .unwrap();

        if extension.to_lowercase() == "png" {
            match read_image_to_byte_array(&path_name){
                Ok(image_bytes) => {
                    let byte_count = image_bytes.len();
                    println!("Image size in bytes {}", byte_count);
                    let blob_c = blob_client
                        .clone()
                        .blob_client(&*container, &*file_name);
                    println!("Sending {} to blob...", file_name);
                    let response = 
                        blob_c.put_block_blob(image_bytes.clone())
                        .content_type("image/png")
                        .await?;
                    
                    match fs::remove_file(path_name) {
                        Ok(()) => println!("File removed successfully"),
                        Err(err) => eprintln!("Failed to remove file! {}", err)
                    }
                }
                Err(error) => {
                    eprintln!("Error reading image file: {}", error);
                }
            }
        }
    }
    Ok(())
}

fn read_image_to_byte_array(path: &str) -> Result<Vec<u8>, Error> {
    let mut file = File::open(path)?;
    let mut buffer = Vec::new();

    file.read_to_end(&mut buffer)?;
    return Ok(buffer);
  }
