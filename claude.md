用 bevy + bevy_extended_ui，来翻译这个项目（E:\\Workspace\\project\\mir2\\Client for Asp），这是一个游戏项目 The Legend of Mir 2 的客户端。

游戏资源目录：E:\Game\Online\Legend of mir


bevy_extended_ui demo (E:\Workspace\project\Rust\bevy_extended_ui-main\examples)

先分析项目结构，然后设计架构，然后开始翻译bevy的特性，貌似只需要2d就行

如果遇到网络请求的数据，则通过模拟数据的方式来处理（尝试生成对应proto文件与数据），在一个rs文件中提供一个方法然后使用消息id来获取对应的数据

资源文件的读取靠 formats image 目录的代码， 这两个可以按照架构重新存放到其它目录
