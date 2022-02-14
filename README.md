# Image size

```rust
/// using `ImageInfoLoader::from_file`
/// loading image info from file
let info = ImageInfoLoader::from_file("path/to/image").unwrap();

/// using `ImageInfoLoader::from_reader`
/// loading image info from Reader(Read+Seek)
let mut reader={...};
let info = ImageInfoLoader::from_reader(&mut reader).unwrap();
```
