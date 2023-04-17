# VPad Server Rust
VPad的Server的Rust实现，作为Rust学习的练手项目以及我的毕业设计

# Quick Start
## Release
你可以通过Release下载最新的二进制压缩包，其中包含如下内容：
1. `log4rs-config.yaml`：日志配置文件，一般情况下无需配置
2. `vpad_server_rust.exe`：主程序
3. `VPadServer Manual.pdf`：服务器端使用说明

软件的运行依赖一个可以创建虚拟MIDI转发设备的应用程序，这里推荐使用loopMIDI。可以进入[官网](https://www.tobias-erichsen.de/software/loopmidi.html)下载安装。

## 编译
如果你是MacOS或Linux，你也可以在Release页面手动下载源码包，使用`cargo run`运行或`cargo build`编译。

我不确定代码在其它平台上是否运行正常，若遇到在其它平台编译出现问题，或某些功能出现问题，可以fork项目，创建自己的分支，或者提交PR。

# TodoList
- [ ] windows平台下使用ffi接入`virtualMidi.dll`，进行虚拟MIDI设备
- [ ] 其它平台下使用`midir`进行MIDI设备虚拟
- [ ] 重新设计琶音器的音量包络，让其语义更加明确
- [x] 修复有时会报Incompleted Message 2的问题
- [x] 添加日志库
- [x] 通过MCU协议支持DAW控制
- [x] 边做边重新编写协议文档（之前的弄丢了）
- [x] 修复BUG：音量包络不应该和`up_note_cnt`关联
- [x] 支持CC Message
- [x] 根据`content_bytes`读取整条消息，而不是依赖当前版本的消息定义
- [x] 支持调制和弯音齿轮 Message
- [x] 支持Arp Message
- [x] 支持Midi Message
- [x] 支持HandShake Message

# Client TodoList
客户端由Android实现，本不应该写在这里，但是目前不方便clone原来的Android项目，就写这吧。

- [ ] 弯音齿轮释放后最后一个提交的PitchWheel Message不是64
- [ ] Pad点击模式：trigger模式和toggle模式
  - trigger模式点击发送midi on，松手发送midi off
  - toggle模式点击发送midi on，再次点击发送midi off
- [x] 修改连接逻辑，CC和轨道界面会使连接断开
- [x] PDST的复制粘贴
