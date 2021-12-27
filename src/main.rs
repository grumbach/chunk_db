pub use xor_name::XorName;
pub use safe_network::types::{Chunk, ChunkAddress};

use std::path::Path;
use bytes::Bytes;

use tokio::fs::File;
use tokio::io::AsyncWriteExt; // for write_all()
use tokio::io::AsyncReadExt; // for read_to_end()

//tmp anyhow
use color_eyre::eyre::Result;

const BIT_TREE_DEPTH: usize = 20;
const CHUNK_STORE_PATH: &str = "/tmp/chunks";
const CHUNK_EXT: &str = ".chunk";

fn address_to_filepath(addr: &ChunkAddress) -> String {
    let xorname = *addr.name();
    let bin = format!("{:b}", xorname);
    let hex = format!("{:x}", xorname);
    let filename = format!("{}{}", hex, CHUNK_EXT);
    let dir_path:String = bin.chars()
        .take(BIT_TREE_DEPTH)
        .map(|c| format!("{}/", c))
        .collect();

    let path = format!("{}/{}/{}", CHUNK_STORE_PATH, dir_path, filename);
    path
}

pub async fn write_chunk(data: &Chunk) -> Result<()> {
    let addr = data.address();
    let path_str = address_to_filepath(&addr);
    let filepath = Path::new(&path_str);
    match filepath.parent() {
        Some(dirs) => {tokio::fs::create_dir_all(dirs).await?;},
        None => {},
    }

    let mut file = File::create(filepath).await?;
    file.write_all(data.value()).await?;
    Ok(())
}

pub async fn read_chunk(addr: &ChunkAddress) -> Result<Chunk> {
    let path_str = address_to_filepath(&addr);
    let filepath = Path::new(&path_str);

    let mut f = File::open(filepath).await?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).await?;

    let bytes = Bytes::from(buffer);
    let chunk = Chunk::new(bytes);
    Ok(chunk)
}

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let chunk = Chunk::new(Bytes::from("hello world!"));
    let addr = &chunk.address();

    write_chunk(&chunk).await?;

    let path = address_to_filepath(addr);
    println!("{}\n", path);

    let read_chunk = read_chunk(addr).await?;

    println!("GOT: {:?}\n", read_chunk.value());

    Ok(())
}
