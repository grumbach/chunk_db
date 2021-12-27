# Chunk DB

A simple chunk store system for the SAFE Network

## Chunks

Chunks are stored as files on the disk in a tree of directories `0` or `1` that represent the first bits of a chunk's address prefix.

We store data in the dir that matches the first `n` (say 20) bits of its name.
As the network splits we can take any data in dirs not in our prefix and delete them and the dirs (delete means store them back on the network and delete locally).

```rust
// chunk bin addr:
0b1001110000100100101100000110000101000011110000000111001000100100110010001001011110111010110010010111001011100110111010010010101101000110110011110001100000000110001111110001101001000110100111101011111000101111011110100000100101100110001100000110000100000101

// chunk hex addr:
0x9c24b06143c07224c897bac972e6e92b46cf18063f1a469ebe2f7a0966306105

// will be stored at path:
"1/0/0/1/1/1/0/0/0/0/1/0/0/1/0/0/1/0/1/1/9c24b06143c07224c897bac972e6e92b46cf18063f1a469ebe2f7a0966306105.chunk"
```
