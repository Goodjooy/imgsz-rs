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

## 支持格式

- `Png`
- `Jpeg`
- `Gif`
- `WebP` 
    > **`webP`格式不保证能解析全部格式，如果是`WebP`格式但是解析失败大小将固定为 100X100**
