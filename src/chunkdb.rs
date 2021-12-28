use safe_network::types::{Chunk, ChunkAddress};
use xor_name::Prefix;

use bytes::Bytes;
use glob::{glob, GlobError};
use std::path::Path;

use tokio::fs::File;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt; // for write_all() // for read_to_end()

use color_eyre::eyre::Result; //tmp anyhow

const BIT_TREE_DEPTH: usize = 20;
const CHUNK_STORE_PATH: &str = "/tmp/chunks";
const CHUNK_EXT: &str = ".chunk";

pub fn address_to_filepath(addr: &ChunkAddress) -> String {
    let xorname = *addr.name();
    let bin = format!("{:b}", xorname);
    let hex = format!("{:x}", xorname);
    let filename = format!("{}{}", hex, CHUNK_EXT);
    let dir_path: String = bin
        .chars()
        .take(BIT_TREE_DEPTH)
        .map(|c| format!("{}/", c))
        .collect();

    let path = format!("{}/{}/{}", CHUNK_STORE_PATH, dir_path, filename);
    path
}

pub async fn write_chunk(data: &Chunk) -> Result<()> {
    let addr = data.address();
    let path_str = address_to_filepath(addr);
    let filepath = Path::new(&path_str);
    if let Some(dirs) = filepath.parent() {
        tokio::fs::create_dir_all(dirs).await?;
    }

    let mut file = File::create(filepath).await?;
    file.write_all(data.value()).await?;
    Ok(())
}

pub async fn read_chunk(addr: &ChunkAddress) -> Result<Chunk> {
    let path_str = address_to_filepath(addr);
    let filepath = Path::new(&path_str);

    let mut f = File::open(filepath).await?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;

    let bytes = Bytes::from(buffer);
    let chunk = Chunk::new(bytes);
    Ok(chunk)
}

pub fn list_all_files() -> Result<Vec<String>> {
    let chunks_path = format!("{}/**/*{}", CHUNK_STORE_PATH, CHUNK_EXT);
    let path = Path::new(&chunks_path);
    let files = glob(&path.display().to_string())?
        .map(|res| res.map(|filepath| filepath.display().to_string()))
        .collect::<Result<Vec<String>, GlobError>>()?;
    Ok(files)
}

pub fn list_files_without_prefix(prefix: Prefix) -> Result<Vec<String>> {
    let all_files = list_all_files()?;

    // get path for matching prefix
    let bit_count = prefix.bit_count();
    let xorname = prefix.name();
    let bin = format!("{:b}", xorname);
    let prefix_dir_path: String = bin
        .chars()
        .take(bit_count)
        .map(|c| format!("{}/", c))
        .collect();
    let prefix_files_path = format!("{}/{}", CHUNK_STORE_PATH, prefix_dir_path);

    // get files outside that path
    let outside_prefix = all_files
        .into_iter()
        .filter(|p| !p.starts_with(&prefix_files_path))
        .collect();
    Ok(outside_prefix)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_write_read_chunk() {
        let chunk = Chunk::new(Bytes::from("test"));
        let addr = &chunk.address();

        write_chunk(&chunk).await.expect("Failed to write chunk.");
        let read_chunk = read_chunk(addr).await.expect("Failed to read chunk.");

        assert_eq!(chunk.value(), read_chunk.value());
    }

    #[tokio::test]
    async fn test_write_read_async_multiple_chunks() {
        let chunk1 = Chunk::new(Bytes::from("test1"));
        let chunk2 = Chunk::new(Bytes::from("test2"));
        let chunk3 = Chunk::new(Bytes::from("test3"));
        let chunk4 = Chunk::new(Bytes::from("test4"));
        let addr1 = &chunk1.address();
        let addr2 = &chunk2.address();
        let addr3 = &chunk3.address();
        let addr4 = &chunk4.address();

        let (res1, res2, res3, res4) = tokio::join!(
            write_chunk(&chunk1),
            write_chunk(&chunk2),
            write_chunk(&chunk3),
            write_chunk(&chunk4),
        );
        res1.expect("error writing chunk1");
        res2.expect("error writing chunk2");
        res3.expect("error writing chunk3");
        res4.expect("error writing chunk4");

        let (read_chunk1, read_chunk2, read_chunk3, read_chunk4) = tokio::join!(
            read_chunk(addr1),
            read_chunk(addr2),
            read_chunk(addr3),
            read_chunk(addr4),
        );

        assert_eq!(chunk1.value(), read_chunk1.expect("error reading chunk 1").value());
        assert_eq!(chunk2.value(), read_chunk2.expect("error reading chunk 2").value());
        assert_eq!(chunk3.value(), read_chunk3.expect("error reading chunk 3").value());
        assert_eq!(chunk4.value(), read_chunk4.expect("error reading chunk 4").value());
    }

    #[tokio::test]
    async fn test_write_read_async_multiple_identical_chunks() {
        let chunk1 = Chunk::new(Bytes::from("test_concurrent"));
        let chunk2 = Chunk::new(Bytes::from("test_concurrent"));
        let chunk3 = Chunk::new(Bytes::from("test_concurrent"));
        let chunk4 = Chunk::new(Bytes::from("test_concurrent"));
        let addr1 = &chunk1.address();
        let addr2 = &chunk2.address();
        let addr3 = &chunk3.address();
        let addr4 = &chunk4.address();

        let (res1, res2, res3, res4) = tokio::join!(
            write_chunk(&chunk1),
            write_chunk(&chunk2),
            write_chunk(&chunk3),
            write_chunk(&chunk4),
        );
        res1.expect("error writing chunk1");
        res2.expect("error writing chunk2");
        res3.expect("error writing chunk3");
        res4.expect("error writing chunk4");

        let (read_chunk1, read_chunk2, read_chunk3, read_chunk4) = tokio::join!(
            read_chunk(addr1),
            read_chunk(addr2),
            read_chunk(addr3),
            read_chunk(addr4),
        );

        assert_eq!(chunk1.value(), read_chunk1.expect("error reading chunk 1").value());
        assert_eq!(chunk2.value(), read_chunk2.expect("error reading chunk 2").value());
        assert_eq!(chunk3.value(), read_chunk3.expect("error reading chunk 3").value());
        assert_eq!(chunk4.value(), read_chunk4.expect("error reading chunk 4").value());
    }
}
