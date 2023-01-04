# VPad Server Rust
VPad的Server的Rust实现，作为Rust学习的练手项目以及我的毕业设计

通过这个项目能学到：

- 扎实Rust基础知识
- 异步rust及tokio
- 使用Rust进行网络编程
- ffi

# TodoList
- [ ] windows平台下使用ffi接入`virtualMidi.dll`，进行虚拟MIDI设备
- [ ] 其它平台下使用`midir`进行MIDI设备虚拟
- [ ] 通过MCU协议支持DAW控制
- [ ] 边做边重新编写协议文档（之前的弄丢了）
- [ ] 重新设计琶音器的音量包络，让其语义更加明确
- [x] 支持CC Message
- [x] 根据`content_bytes`读取整条消息，而不是依赖当前版本的消息定义
- [x] 支持调制和弯音齿轮 Message
- [x] 支持Arp Message
- [x] 支持Midi Message
- [x] 支持HandShake Message