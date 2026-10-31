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
    └── index.db            # basic info, metadata and computed data like hash, colors...
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

### Database Schema

The generated `index.db` contains columns:

| Column   | Type    | Description                             | Example                   |
| -------- | ------- | --------------------------------------- | ------------------------- |
| id       | INTEGER | Auto-incrementing                       | `42`                      |
| path     | TEXT    | Relative path to image                  | `"rel_path/to/image.png"` |
| name     | TEXT    | Image name                              | `"image"`                 |
| format   | TEXT    | Image format (PNG, JPG, WEBP)           | `"PNG"`                   |
| size     | INTEGER | File size in bytes                      | `2478406`                 |
| width    | INTEGER | Image width in pixels                   | `3840`                    |
| height   | INTEGER | Image height in pixels                  | `2160`                    |
| created  | INTEGER | Creation timestamp (ms)                 | `1792321201`              |
| modified | INTEGER | Modification timestamp (ms)             | `1792321201`              |
| rating   | INTEGER | User rating (0 to 5)                    | `3`                       |
| tags     | TEXT    | Tags separate by pipe                   | `"girl\|beach\|sunset"`   |
| hash     | TEXT    | BLAKE3 hash of pixels (hex 64)          | `"35872f721...684"`       |
| color_1  | INTEGER | Dominant color #1 in decimal            | `12431782`                |
| color_2  | INTEGER | Dominant color #2 in decimal (may null) | `7630177`                 |
| color_3  | INTEGER | Dominant color #3 in decimal (may null) | `0`                       |
| color_4  | INTEGER | Dominant color #4 in decimal (may null) | `NULL`                    |

### Supported Image Formats

| `PNG` | `JPG` | `WEBP` |
| ----- | ----- | ------ |
