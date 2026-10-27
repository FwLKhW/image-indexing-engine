# Image Indexing Engine

### Usage

```sh
image-indexing-engine "home/david/Pictures/girlfriend/"
```

### Output

Under `home/david/Pictures/girlfriend/`:

```sh
.eyeris/
├── thumbnails/             # 512px-bounded thumbnails
│   ├── 1dn32hf...rbs.png
│   ├── j5f49hf...asd.png
│   └── ...
└── images.db               # basic info, system metadata and some metadata standards.
```

### Project Structure

```sh
src/
├── main.rs           # CLI
├── lib.rs
├── color.rs          # dominant colors extraction
├── db.rs             # db operations
├── hash.rs           # create unique identity for image using pixels + format
├── index.rs          # indexing pipeline include thumbnailing
├── paths.rs
├── scan.rs           # scan images recursively
└── thumbnail.rs      # resize 512-bound
```

### Database Schema

`images.db` contain single table `images`:

```sql
CREATE TABLE IF NOT EXISTS images (
    path        TEXT        PRIMARY KEY,            -- rel_path/to/image.png
    name        TEXT        NOT NULL,               -- image
    format      TEXT        NOT NULL,               -- PNG, JPG, WEBP
    size        INT         NOT NULL,               -- in bytes
    width       INT         NOT NULL,               -- 3840
    height      INT         NOT NULL,               -- 2160
    created     INT         NOT NULL,               -- 1792552976580
    modified    INT         NOT NULL,               -- 1792552976580
    rating      INT         DEFAULT 0,              -- 0 to 5
    description TEXT        DEFAULT '',             -- 'A beautiful girl ...'
    tags        TEXT        DEFAULT '[]',           -- ['girl', 'beach', 'sunset']
    hash        TEXT        DEFAULT '',             -- blake3 hash of rgb8 pixels value
    palette     TEXT        DEFAULT '[]'            -- List of Oklab (3 floats)
);
```
