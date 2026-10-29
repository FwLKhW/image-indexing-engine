# Image Indexing Engine

> [!WARNING]
> Incremental indexing WIP!

### Usage

```sh
Usage: index.exe [OPTIONS] <FOLDER_PATH>

Arguments:
  <FOLDER_PATH>  Target folder

Options:
-t, --thumbnail  Enable generate thumbnails
-i, --index      Enable basic indexing
-e, --extended   Enable extended indexing (hash, palette)
```

#### Example

```sh
index "~/Pictures/girlfriend/" -tie
```

#### Output

Under `~/Pictures/girlfriend/`:

```sh
.eyeris/
├── thumbnails/             # 512px-bounded thumbnails
│   ├── 1dn32hf...rbs.png
│   ├── j5f49hf...asd.png
│   └── ...
└── index.csv               # basic info, metadatas and computed data like hash, colors...
```

### Project Structure

```sh
src/
├── main.rs           # CLI
├── lib.rs
├── color.rs          # dominant colors extraction
├── csv.rs            # CSV export operations
├── hash.rs           # create unique identity for image using pixels + format
├── index.rs          # indexing pipeline include thumbnailing
├── metadata.rs       # read metadata (rating, tags, description)
├── paths.rs
├── scan.rs           # scan images recursively
└── thumbnail.rs      # resize 512-bound
```

### CSV Schema

The generated `index.csv` contains columns for direct analysis in Python Pandas:

| Column      | Type        | Description                                                | Example                                                                                                               |
| ----------- | ----------- | ---------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| id          | Integer     | Auto-incrementing                                          | 42                                                                                                                    |
| path        | String      | Relative path to image                                     | "rel_path/to/image.png"                                                                                               |
| name        | String      | Image name                                                 | "image"                                                                                                               |
| format      | String      | Image format (PNG, JPG, WEBP)                              | "PNG"                                                                                                                 |
| size        | Integer     | File size in bytes                                         | 2478406                                                                                                               |
| width       | Integer     | Image width in pixels                                      | 3840                                                                                                                  |
| height      | Integer     | Image height in pixels                                     | 2160                                                                                                                  |
| created     | Integer     | Creation timestamp (ms)                                    | 1792321201                                                                                                            |
| modified    | Integer     | Modification timestamp (ms)                                | 1792321201                                                                                                            |
| rating      | Integer     | User rating (0 to 5)                                       | 3                                                                                                                     |
| description | String      | Image description                                          | "A random image"                                                                                                      |
| tags        | JSON String | List of tags                                               | "['girl', 'beach', 'sunset']                                                                                          |
| hash        | String      | BLAKE3 hash of rgba16 pixel values                         | "35872f721295a1aff5c658ee82f046f0523b96919b2f9859f418b56d35422684"                                                    |
| palette     | JSON String | List of dominant colors in Oklab format ([[l, a, b], ...]) | "[[0.23075044,2.7358425e-8,4.370804e-9],[0.60934067,0.020011054,0.023310097],[0.83049333,-0.0034050737,0.018965157]]" |

### Supported Image Formats

| `PNG` | `JPG` | `WEBP` |
| ----- | ----- | ------ |
