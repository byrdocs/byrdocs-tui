# BYR Docs README

> BYR Docs 工作目录

## 文件结构

建议的文件结构为

```shell
/path/to/BYRDOCS/ # 如 ~/BYRDOCS
├── .cache # 必需
├── README.md # 非必需，但会自动生成
├── archive # 必需
├── completion # 非必需
├── stockpile # 必需
└── tui # 非必需
```

其中标注为 `必需` 的部分，是在TUI程序进行初始化之际就生成的，**请勿轻易改动**。

- `.cache` 是一个 **缓存** 目录。下载自 BYR Docs 中的文件将暂存于此。如果一段时间内未被使用，程序将自动清理这些文件——无需手动操作。
- [`archive`](https://github.com/byrdocs/byrdocs-archive) 目录保存所有文件的 **元信息**。在编辑元信息前，请先 fork 此仓库到你自己的 GitHub 账号中，修改完成后再向本仓库提 Pull Request。
- [`stockpile`](https://github.com/byrdocs/byrdocs-scripts) 是 **文件暂存库** 也是 **脚本库**。这里保存了一些处理文件时可能用到的脚本。与 `.cache` 不同，它保存的是「你计划上传到 BYR Docs 中的文件」，而不是从 BYRDOCS 中下载的文件。

而 `非必需` 的部分不会自动生成。如有需要，你可以手动 `git clone` 它们。

- [`completion`](https://github.com/byrdocs/byrdocs-completion) 是 BYR Docs 的子项目，旨在为部分考题编辑答案，或者重新排版一些质量不佳的考题文件。如果你有兴趣参与这个子项目，欢迎提 Pull Request。
- [`tui`](https://github.com/byrdocs/byrdocs-tui) 是 BYR Docs 的集成命令行工具。如果你有意向帮我们改进功能、改良 UI 或者提供新的 themes，欢迎提 Pull Request。

## 参考文档

详情请查看 [byrdocs tui README](https://github.com/byrdocs/byrdocs-tui/blob/master/docs/README.md)。
