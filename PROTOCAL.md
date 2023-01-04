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
消息格式中的换行是为了可读性，实际的消息并无换行
```
content_bytes: int2
op: int1
content_bytes bytes
```

为了消息的可扩展性，后版本的消息可能在前版本的消息基础上添加字段，所以，不论你当前的版本是否需要用到那么多字段，你都必须将`content_bytes`中的字节数量读完，然后才进入下一条消息的解析。

在早期版本中，对后期版本新增字段表现出的行为是——忽略。

## HandShake Message

```
-- version: 1
content_bytes: int2
1
name: string        // 名字
platform: string    // 平台
```

## Midi Message
```
-- version: 1
content_bytes: int2
2
note: int1          // 音符
velocity: int1      // 力度
state: int1         // 状态
```

## Arp Message
```
-- version: 1
content_bytes: int2
3
note: int1
velocity: int1
state: int1
method: int1,               // 琶音方式
rate: int1,                 // 琶音速率
swing_pct: int1,            // 摇摆程度，0..=100
up_note_cnt: int1,          // 上行音符数
velocity_automation: int1,  // 力度包络
dynamic_pct: int2,          // 动态范围
bpm: int2                   // 琶音bpm
```

下面是ArpMessage中的部分字段的枚举表，字段只限制了从名字来看，该字段该有的行为，比如`METHOD_UP`，代表该琶音是上行的琶音，至于服务端如何解析它，不做限制，所以有可能出现同样的琶音设置在不同的服务端上行为不一致的情况。


### method 琶音方法
```
METHOD_NO_METHOD = 0
METHOD_UP = 1           // 上行琶音
METHOD_DOWN = 2         // 下行琶音
METHOD_UP_DOWN = 3      // 上行后下行
METHOD_DOWN_UP = 4      // 下行后上行
METHOD_3CHORD = 5       // 三和弦
METHOD_7CHORD = 6       // 七和弦
METHOD_3MINCHORD = 7    // 小三和弦
METHOD_7MINCHORD = 8    // 小七和弦
METHOD_11CHORD = 9      // 大十一和弦
METHOD_13CHORD = 10     // 大十三和弦
METHOD_11MINCHORD = 11  // 小十一和弦
METHOD_13MINCHORD = 12  // 小十三和弦
```

### rate 琶音速率
```
RATE_1_1 = 0
RATE_1_2_D = 1
RATE_1_1_T = 2
RATE_1_2 = 3
RATE_1_4_D = 4
RATE_1_2_T = 5
RATE_1_4 = 6
RATE_1_8_D = 7
RATE_1_4_T = 8
RATE_1_8 = 9
RATE_1_16_D = 10
RATE_1_8_T = 11
RATE_1_16 = 12
RATE_1_32_D = 13
RATE_1_16_T = 14
RATE_1_32 = 15
RATE_1_64_D = 16
RATE_1_32_T = 17
RATE_1_64 = 18
RATE_1_64_T = 19
```

> `RATE_<x>_<y>[_z]`，x和y是必选项，x固定为1，y代表一个拍子被分成几份，简单来说，`1_8`代表八分音符。 z是可选项，它是后缀，`D`代表是一个附点音符的时值，`T`代表是三连音的时值

### velocity_automation 力度包络
```
VELOCITY_NO_AUTOMATION = 0
VELOCITY_UP = 1           // 上行
VELOCITY_DOWN = 2         // 下行
VELOCITY_UP_DOWN = 3      // 上行后下行
VELOCITY_DOWN_UP = 4      // 下行后上行
VELOCITY_STEP = 5         // 步进
VELOCITY_RANDOM = 6       // 随机
```

## Chord Message

`-`

## PitchWheel Message
```
content_bytes: int2
5
pos: int1                 // 当前弯音轮位置
prev_pos: int1            // 前一个弯音轮位置
```

> 大部分设备上的弯音轮是物理滚轮，所以，不可能从一个位置跳跃到另一个不相邻的位置。 一些低端设备上使用触控条来模拟弯音轮，当你连续点击触控条上的两个不相邻位置时，所产生的行为是软件模拟了从一个位置到另一个位置的滚动。所以PitchWheel Message加入`prev_pos`字段，来记录上一个弯音位置，默认是64（弯音轮的中间位置）。 服务端可以选择使用该字段模拟弯音轮的滚动，也可以选择不实现。

> 弯音轮释放时是会回弹到0的，这一行为由客户端实现，客户端在用户从弯音轮松手后会发送一条到中间位置64的消息。