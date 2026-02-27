用 bevy + bevy_extended_ui，来翻译这个项目（E:\\Workspace\\project\\mir2\\Client for Asp;G:\\workspace\\game\\Client for Asp），这是一个游戏项目 The Legend of Mir 2 的客户端。

游戏资源目录：E:\Game\Online\Legend of mir

bevy_extended_ui path:
(
1: E:\Workspace\project\Rust\bevy_extended_ui
2: G:\workspace\Rust\bevy_extended_ui\examples
)

bevy_extended_ui

examples 示例项目

CSS_USAGE.md ${path}\CSS_USAGE.md
本文件反映了 src/styles/parser.rs 实际解析和应用的规则。不支持的属性或值将被静默忽略。
EVENT.md ${path}\EVENT.md
本文档描述了 bevy_extended_ui 提供的 HTML 事件绑定以及在 Rust 中接收的有效载荷。
LANGUAGE.md ${path}\LANGUAGE.md
概述 这个 crate 可以通过将 HTML 中的占位符替换为翻译后的字符串和运行时变量来本地化 HTML 内容。只有当启用了语言后端功能时，本地化才会生效。
WIDGETS.md
本文件描述了项目中所有组件，它们对应的 Rust 结构体，以及它们在 HTML 源代码中的呈现方式。

先分析项目结构，然后设计架构，然后开始翻译bevy的特性，貌似只需要2d就行

如果遇到网络请求的数据，则通过模拟数据的方式来处理（尝试生成对应proto文件与数据），在一个rs文件中提供一个方法然后使用消息id来获取对应的数据

资源文件的读取靠 formats image 目录的代码， 这两个可以按照架构重新存放到其它目录
