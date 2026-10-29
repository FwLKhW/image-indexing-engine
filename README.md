# Image Indexing Engine

> [!WARNING]
> Incremental indexing WIP!

&nbsp;

### 💫 Usage

```sh
iie <COMMAND>

Commands:
  thumb  Generate thumbnails
  index  Indexing
  write  Write metadata to image
```

### ✍️ Examples

#### 🖼️ Generate thumbnails

```sh
iie thumb "~/Pictures/girlfriend/"
```

Output:

```sh
girlfriend/
└── .eyeris/
    └── thumbnails/         # 512px-bounded thumbnails
        ├── 1dn32hf...rbs.png
        ├── j5f49hf...asd.png
        └── ...
```

#### 🔍 Indexing

```sh
iie index "~/Pictures/girlfriend/"
```

Output:

```sh
girlfriend/
└── .eyeris/
    └── index.csv           # basic info, metadatas and computed data like hash, colors...
```

#### 📝 Write metadata

```sh
iie write "~/Pictures/girlfriend/2022-02-02.png" rating 5
iie write "~/Pictures/girlfriend/2022-02-02.png" tag "dating"
```

### Project Structure

```sh
src/
├── main.rs           # CLI
├── lib.rs
├── color.rs          # dominant colors extraction
├── csv.rs            # CSV export operations
├── hash.rs           # hash image using pixels value
├── index.rs          # indexing
├── metadata.rs       # read metadata (rating, tags)
├── paths.rs
├── scan.rs           # scan images recursively
└── thumbnail.rs      # resize 512-bound
```

### CSV Schema

The generated `index.csv` contains columns for direct analysis in Python Pandas:

| Column   | Type | Description                         | Example                 |
| -------- | ---- | ----------------------------------- | ----------------------- |
| id       | int  | Auto-incrementing                   | 42                      |
| path     | str  | Relative path to image              | "rel_path/to/image.png" |
| name     | str  | Image name                          | "image"                 |
| format   | str  | Image format (PNG, JPG, WEBP)       | "PNG"                   |
| size     | int  | File size in bytes                  | 2478406                 |
| width    | int  | Image width in pixels               | 3840                    |
| height   | int  | Image height in pixels              | 2160                    |
| created  | int  | Creation timestamp (ms)             | 1792321201              |
| modified | int  | Modification timestamp (ms)         | 1792321201              |
| rating   | int  | User rating (0 to 5)                | 3                       |
| tags     | str  | Tags seperate by pipe               | "girl\|beach\|sunset"   |
| hash     | str  | BLAKE3 hash of pixels (hex 64)      | "35872f721...684"       |
| color_1  | str  | Dominant color #1 in hex            | "#E17591"               |
| color_2  | str  | Dominant color #2 in hex (may null) | "#E17591"               |
| color_3  | str  | Dominant color #3 in hex (may null) | "#E17591"               |
| color_4  | str  | Dominant color #4 in hex (may null) | ""                      |

### Supported Image Formats

| `PNG` | `JPG` | `WEBP` |
| ----- | ----- | ------ |
