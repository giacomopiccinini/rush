# rush

A Swiss-army knife CLI tool for data inspection and manipulation, written in Rust.

## Table of Contents
- [Overview](#overview)
- [Installation](#installation)
- [Command Categories](#command-categories)
  - [Audio Commands](#audio-commands)
  - [Image Commands](#image-commands)
  - [Video Commands](#video-commands)
  - [File Commands](#file-commands)
  - [Table Commands](#table-commands)

## Overview

`rush` is a command-line utility that provides various tools for working with multimedia files and data tables. It's designed to be fast and efficient, leveraging parallel processing where possible. 

Specifically, it is capable of handling images, videos, audios, tabular files and (to some extent) "generic" files and directories. Restrictions might apply on admissible file formats, for instance only *.wav* audio files can be resampled. 

The syntax is consistent across modalities and follows the pattern
```bash
rush <MEDIA> <COMMAND> <OPTIONS>
```
as in `rush image summary mypic.png`. 

## Installation
```bash
cargo install --path .
```
Be aware that some extra dependecies are needed, mostly related to FFMpeg. On Debian-like systems, ensure you run first
```bash
sudo apt update && apt install -y ffmpeg libavformat-dev libavutil-dev libavcodec-dev libavfilter-dev libavdevice-dev libclang-dev
```
If the standard cargo installation fails, please consider using the provided Dockerfile. To build that run

```bash
docker build --tag rush .
```
Notice, however, that since rush is meant to interact with directories and files on your computer, these are not available straight away to a Docker container running rush. Hence, make sure to mount them before running a command, for instance
```bash
docker run -v "$(pwd)/test-directory:/app/test-directory" rush <COMMAND> /app/test-directory
```
## Command Categories

### Audio Commands

#### `audio summary`
Get metadata about audio files (a single file or a directory).

**Supported Extensions**: `.mp3`, `.wav`, `.ogg`, `.flac`, `.aac`, `.m4a`  
**Input**: Can be a single file or directory (recursive)

```bash
rush audio summary <target>
```

Example:
```bash
rush audio summary music/
```

Output:
```
Total files: 42
Total Duration: 02:15:30
Average Duration: 193 s
Sample Rates: {44100, 48000} Hz
Channels: {1, 2}
Bit Depths: {16, 24}
Unique durations: 42
Min duration: 120.5 s
Max duration: 345.2 s
```

#### `audio split`
Split audio files into chunks of specified duration.

**Supported Extensions**: `.wav` only  
**Input**: Can be a single file or directory (recursive)

```bash
rush audio split <input> <chunk_duration> <output> [--delete-original]
```

Example:
```bash
rush audio split long.wav 30 chunks/ --delete-original
```

This will split long.wav into 30-second chunks and save them in the `chunks/` directory. The original `long.wav` file will be deleted.

#### `audio resample`
Change the sample rate of audio files.

**Supported Extensions**: `.wav` only  
**Input**: Can be a single file or directory (recursive)

```bash
rush audio resample <input> <sr> <output> [--overwrite]
```

Example:
```bash
rush audio resample input.wav 44100 output.wav
```

#### `audio trim`
Trim audio files to a specified length.

**Supported Extensions**: `.wav` only  
**Input**: Can be a single file or directory (recursive)

```bash
rush audio trim <input> <length> <output> [--offset <seconds>] [--overwrite]
```

Example:
```bash
rush audio trim input.wav 60 output.wav --offset 30
```

### Image Commands

#### `image summary`
Get metadata about image files.

**Supported Extensions**: `.jpg`, `.jpeg`, `.png`, `.bmp`, `.gif`, `.tiff`  
**Input**: Can be a single file or directory (recursive)

```bash
rush image summary <target>
```

Example:
```bash
rush image summary photos/
```

Output:
```
Total files: 156
Unique (height, width) pairs: {(1080, 1920), (800, 600), (3024, 4032)}
```

#### `image resize`
Resize images to specified dimensions.

**Supported Extensions**: `.jpg`, `.jpeg`, `.png`, `.bmp`, `.gif`, `.tiff`  
**Input**: Can be a single file or directory (recursive)

```bash
rush image resize <input> <height> <width> <output> [--overwrite]
```

Example:
```bash
rush image resize input.jpg 1080 1920 output.jpg
```

#### `image tessellate`
Split images into a grid of smaller images.

**Supported Extensions**: `.jpg`, `.jpeg`, `.png`, `.bmp`, `.gif`, `.tiff`  
**Input**: Can be a single file or directory (recursive)

```bash
rush image tessellate <input> <n_vertical> <n_horizontal> <output> [--delete-original]
```

Example:
```bash
rush image tessellate photo.jpg 2 3 tiles/
```

This splits the image into a 2×3 grid (6 pieces).

### Video Commands

#### `video summary`
Get metadata about video files.

**Supported Extensions**: `.ts`, `.mp4`, `.mkv`, `.mov`  
**Input**: Can be a single file or directory (recursive)

```bash
rush video summary <target>
```

Example:
```bash
rush video summary videos/
```

Output:
```
Total files: 12
Total duration: 3600.5
Unique durations: {120, 240, 360}
Unique (height, width) pairs: {(1080, 1920), (720, 1280)}
Unique FPS: {(30, 1), (60, 1)}
```

### File Commands

#### `file count`
Count files and directories in a given path.

**Supported Extensions**: All files  
**Input**: Can be a single file or directory (non-recursive, only immediate children)

```bash
rush file count <target>
```

Example:
```bash
rush file count documents/
```

Output:
```
Files: 145
Directories: 12
```

### Table Commands

#### `table schema`
Display the schema of a CSV or Parquet file.

**Supported Extensions**: `.csv`, `.parquet`  
**Input**: Single file only (directories not supported)

```bash
rush table schema <input>
```

Example:
```bash
rush table schema data.csv
```

Output:
```
Schema:
name: Name, field: String
name: Age, field: Int64
name: Department, field: String
name: Salary, field: Int64
```

## License

MIT License - See [LICENSE](LICENSE) for details.
