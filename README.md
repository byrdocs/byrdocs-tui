# BYRDOCS CLI

> BYRDOCS 集成命令行工具

- 以命令行应用实现文件整理。
- 以 .json 文件存储文档元信息，并存储于 [archive](https://github.com/byrdocs/byrdocs-archive)。
- 支持文档检视、封面提取、文件去重、元信息查询等各种功能。

## Configuration

使用本应用之前，您需要在 `~/.config/` 中建立一个 `byrdocscli.toml` 文件。

以下内容仅作参考，您可根据实际情况进行调整：

```toml
resources_dir="~/BYRDOCS/resources"
stockpile_dir="~/BYRDOCS/stockpile"
archive_dir="~/BYRDOCS/archive"
books_dir="~/BYRDOCS/resources/books"
tests_dir="~/BYRDOCS/resources/tests"
docs_dir="~/BYRDOCS/resources/docs"
covers_dir="~/BYRDOCS/resources/covers"

generate_jpg=true
generate_png=false
generate_webp=true

pdf_viewer="evince"
zip_viewer="ark"
```

## Usage
