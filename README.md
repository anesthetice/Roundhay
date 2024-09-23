### Roundhay - a local network media server

#### Archived
Improved version, called Silvus

#### Usage

* any file that is placed inside the ./res directory will be available to download
* to make a file visible, you must create an entry in "index.json" (same directory as the binary)

index.json example:

{
  "units": [
    {
      "title": "Series - s01e01",
      "description": "Lorem Ipsum",
      "year": 2019,
      "languages": [
        "English"
      ],
      "subtitles": [
        "English"
      ],
      "resolution": {
        "width": 1920,
        "height": 1080
      },
      "encoding": "H265",
      "size": 686.8,
      "path": "Series-e01.mkv"
    },
    {
      "title": "Movie",
      "description": "Lorem Ipsum",
      "year": 2022,
      "languages": [
        "English",
      ],
      "subtitles": [
        "English",
        "French",
      ],
      "resolution": {
        "width": 1920,
        "height": 1080
      },
      "encoding": "H264",
      "size": 4000.0,
      "path": "Movies/Movie.mkv"
    },
  ]
}
