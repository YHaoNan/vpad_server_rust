# VPad通信协议
## 类型
### int1
占用1字节 无符号补码数
### int2
占用2字节 无符号补码数
### string
UTF-8编码 最大字节长度255

```
<n: int1> <nth of bytes>
```
## 消息格式
```
<content_bytes: int2> <op: int1> <content_bytes bytes>
```