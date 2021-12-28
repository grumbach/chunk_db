use bytes::Bytes;
use safe_network::types::Chunk;
use xor_name::Prefix;

use color_eyre::eyre::Result; //tmp anyhow

mod chunkdb;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    // test basic write/read
    let chunk = Chunk::new(Bytes::from("hello world!"));
    let addr = &chunk.address();
    chunkdb::write_chunk(&chunk).await?;

    let path = chunkdb::address_to_filepath(addr);
    println!("{}\n", path);

    let read_chunk = chunkdb::read_chunk(addr).await?;
    println!("GOT: {:?}\n", read_chunk.value());
    assert_eq!(chunk.value(), read_chunk.value());

    // write more chunks
    let chunk1 = Chunk::new(Bytes::from("hello world!1"));
    let chunk2 = Chunk::new(Bytes::from("hello world!2"));
    let chunk3 = Chunk::new(Bytes::from("hello world!3"));

    chunkdb::write_chunk(&chunk1).await?;
    chunkdb::write_chunk(&chunk2).await?;
    chunkdb::write_chunk(&chunk3).await?;

    // test prune
    let files = chunkdb::list_all_files()?;
    println!("ALL FILES: {:#?}", files);
    let prefix = Prefix::new(4, *addr.name());
    println!("PREFIX: {:?}", prefix);
    let prune_files = chunkdb::list_files_without_prefix(prefix)?;
    println!("PRUNE FILES: {:#?}", prune_files);

    Ok(())
}
