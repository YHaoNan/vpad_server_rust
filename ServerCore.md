# ServerCore
VPadServerCore（简称core）只提供以下基本功能：
1. 绑定到固定TCP端口，以接受客户端连接
2. 解析客户端消息并处理
3. 与固定的MIDI输出端口交互

# RunMode
## StandaloneMode
当用户运行`vpadcore`时，若不传入任何参数，进入StandaloneMode，此时core需要询问用户一些基本的信息，比如连接到哪个MIDI输出端口，然后给用户打印出一个连接二维码。

## CoreMode
当用户为`vpadcore`传入参数时，进入CoreMode，此时，我们期待core由携带的参数进行配置。这样调用core的有可能是：
1. 高级用户：它们不想每一次都被询问相同的问题
2. 某种GUI程序：可能是为习惯了GUI的用户提供的GUI启动器，帮助用户更加简单的使用系统

```text
Usage:
vpadcore <-i Instruemnt MIDI output port> <-c Control MIDI output port>
         [-l Log Level(Default to INFO)]
```

## 共同规约
不论是StandaloneMode还是CoreMode，vpadcore在遇到任何阻止它正常运行的问题时都应该崩溃，比如：
1. 无法连接到指定的output port
2. 无法绑定在TCP端口上
3. 运行时crash

所以，如果你想要开发一个GUI启动器，你可以通过检测`vpadcore`进程是否结束来判断当前程序的状态。

vpadcore会向stdout输出任何日志。
