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

```
--version: 1
content_bytes: int2
4
note: int1          // 音符
velocity: int1      // 力度
state: int1         // 状态
chord_type: int1    // 和弦类型
chord_level: int1   // 和弦级数
transpose: int1     // 转位
arp_delay: int1     // 琶音程度   百分数   代表当前一个拍的百分之多少琶音完成
                    // 我们假设bpm=130，那么一拍就是0.461秒，若arp_delay=50，那么代表在0.461*50%=0.2305秒之内完成琶音
                    // 也就是说，和弦内的所有音符，在0.2305秒之内被均匀的放出，顺序是自底向上
```

transpose

### chord_type
```
CHORD_TYPE_MAJOR = 0   // 大和弦  
CHORD_TYPE_MINOR = 1   // 小和弦  使用b3, b7音
CHORD_TYPE_DOM   = 2   // 属和弦  使用b7音 （从7和弦开始）
CHORD_TYPE_AUG   = 3   // 增和弦  使用#5音
CHORD_TYPE_DIM   = 4   // 减和弦  使用b3, b5音
CHORD_TYPE_SUS2  = 5   // 挂2和弦 3音变2音
CHORD_TYPE_SUS4  = 6   // 挂4和弦 3音变4音
CHORD_TYPE_ADD6  = 7   // 加6和弦 在原始和弦上加高6音
CHORD_TYPE_ADD9  = 8   // 加9和弦 在原始和弦上加高9音
```

### chord_level
```
CHORD_LEVEL_3    = 0   // 3和弦  0, 4, 7
CHORD_LEVEL_7    = 1   // 7和弦  0, 4, 7, 11
CHORD_LEVEL_9    = 2   // 9和弦  0, 4, 7, 11, 14
CHORD_LEVEL_11   = 3   // 11和弦 0, 4, 7, 11, 14, 17
CHORD_LEVEL_13   = 4   // 13和弦 0, 4, 7, 11, 14, 17, 21
```


## PitchWheel Message
```
content_bytes: int2
5
pos: int1                 // 当前弯音轮位置
prev_pos: int1            // 前一个弯音轮位置
```

> 大部分设备上的弯音轮是物理滚轮，所以，不可能从一个位置跳跃到另一个不相邻的位置。 一些低端设备上使用触控条来模拟弯音轮，当你连续点击触控条上的两个不相邻位置时，所产生的行为是软件模拟了从一个位置到另一个位置的滚动。所以PitchWheel Message加入`prev_pos`字段，来记录上一个弯音位置，默认是64（弯音轮的中间位置）。 服务端可以选择使用该字段模拟弯音轮的滚动，也可以选择不实现。

> 弯音轮释放时是会回弹到0的，这一行为由客户端实现，客户端在用户从弯音轮松手后会发送一条到中间位置64的消息。

## CC Message
```
content_bytes: int2
7
channel: int1             // 控制哪一个CC，这里由于历史原因所以不方便改名
value: int1               // CC的值
```

## Control Message
```
content_bytes: int2
8
operation: int1           // 要执行的控制操作
state: int1               // 执行后的状态
auto_close: int1          // 是否自动关闭
```

> 控制信息控制的每一个东西都具有开关两种状态（MCU协议设计如此），state设置它的状态，为0是关，为1是开。
> auto_close指定是否自动在开启后发送一条state=0的消息

### Operations
```text
OP_PLAY = 0             // 播放
OP_STOP = 1             // 停止
OP_RECORD = 2           // 录制
OP_UNDO = 3             // 撤销
OP_REDO = 4             // 重做
OP_LOOP = 5             // 开启循环
OP_SAVE = 6             // 保存
OP_ZOOM = 7             // 缩放
OP_CURSOR_L = 8         // 忘了
OP_CURSOR_R = 9
OP_CURSOR_U = 10
OP_CURSOR_D = 11
OP_CLICK = 12             // 节拍器
OP_TRACK_BANK_LEFT = 13   // 轨道左移8个
OP_TRACK_BANK_RIGHT = 14  // 轨道右移8个
```

## TrackMessage
```
content_bytes: int2
9
nth: int1           // 轨道
state: int1         // 状态
value: int1         // 轨道音量值 在`state == STATE_FADER_VALUE_CHANGED`时生效
```

### State
```text
STATE_FADER_UP = 0               // 推子抬起
STATE_FADER_DOWN = 1             // 推子按下
STATE_FADER_VALUE_CHANGED = 2    // 推子值改变（与value变量联动）
STATE_SOLO_ON = 3                // SOLO开启
STATE_SOLO_OFF = 4               // SOLO关闭
STATE_MUTE_ON = 5                // MUTE开启
STATE_MUTE_OFF = 6               // MUTE关闭
STATE_REC_ON = 7                 // 录制开启
STATE_REC_OFF = 8                // 录制关闭
```

# 行为规范
行为规范是VPadServer最好实现的行为，并不是硬性规定，VPadServer实现者可以合理的解释各种字段的语义，只要不会产生让用户迷惑的效果即可。

## Arp Message
出于实用性考虑，VPad只支持4 4拍的音乐，Arp以4 4拍为基准。

### ArpMessage Rate
Rate控制琶音的速率，`RATE_{x}_{y}`中，$\frac{x}{y}$代表琶音中的每个音符占一小节的多少，由于$x$始终为1，所以就是$\frac{1}{y}$，你也可以理解为琶音中的每个音符是一个$y$分音符。

Rate需要与`bpm`联动才有意义，假如你的`bpm`字段是`130`，`rate`是`RATE_1_4`，那么每一个琶音音符的长度应该为：

```
一拍的时间         = 60 / bpm
一小节的时间       = 一拍的时间 * 4
一个琶音音符的时间 = 一小节的时间 / y
```

一些Rate是带后缀的，`_D`后缀代表该音符是附点音符，一个具有`_D`后缀的音符，它的长度等于不具有后缀的音符的长度的1.5倍。

`_T`代表三连音音符，它代表在$y' = y/2$的音符`RATE_{x}_{y'}`长度的$1/3$，简单来说`RATE_1_1`的音符长度中能装下三个$RATE_1_2_T$。

### ArpMessage UpNoteCnt
控制音符改变的数量，由于历史原因，该变量被明明为`up_note_cnt`，但它并不代表上行音符的数量。

当你按下arp时，第一个触发的是你按下的那个音符，接下来，音符将改变`up_note_cnt - 1`次，这个过程结束后，回到你按下的那个音符。

该属性与音符如何改变无关，`method`属性才定义音符如何改变。

### ArpMessage Method
控制琶音音符的走向，与`up_note_cnt`属性联动。

- `METHOD_UP`，音符每次改变上升一个八度
- `METHOD_DOWN`，每次下降一个八度
- `METHOD_UPDOWN`，以八度为单位，先上升再下降
- `METHOD_DOWNUP`，以八度为单位，先下降再上升
- `METHOD_{n}CHORD`，以按下音为根音，每次上升都是当前音上面最近一个和弦内音，和弦为n大和弦
- `METHOD_{n}MINCHORD`，以按下音为根音，每次上升都是当前音上面最近一个和弦内音，和弦为n小和弦

`up_note_cnt`决定了音符改变的次数，如`up_note_cnt`为5，`method`为`METHOD_UP`，按下的音为12，琶音器将产生的音符序列为`12, 24, 36, 48, 60, 12, 24, 36, 48, 60, ...`

有一些琶音模式不是单调上升或下降的的，`UPDOWN`和`DOWNUP`就是，这种情况下，考虑第`i`个音符（i从1开始），当$up\_note\_cnt / i == 1$处，琶音器开始转弯。

也就是说，当`up_note_cnt=5`时，在第3个音符处拐弯，`up_note_cnt=6`时，在第四个音符处拐弯。

### ArpMessage VelocityAutomation
`velocity_automation`用于控制琶音器的力度自动化，力度变化曲线以小节为周期单位。

- `VELOCITY_UP`：力度在小节内上升
- `VELOCITY_DOWN`：力度在小节内下降
- `VELOCITY_UP_DOWN`：上升后下降
- `VELOCITY_DOWN_UP`：下降后上升
- `VELOCITY_STEP`：一强一弱
- `VELOCITY_RANDOM`：随机

力度变化的范围由`dynamic_pct`属性控制。

以小节为周期单位的意思是`RATE_1_64`下，`velocity_automation`的周期是64个音符。

具有附点的RATE琶音有点特殊，因为小节内最后会空出一个不足以容纳一个音符的空位，我们采取的办法是从这里开始新一轮的力度周期。

严谨一点说，当RATE为`RATE_{x}_{y}_D`时，以$floor(1/(1/{y} + 1/(y\times 2)))$个音符为力度周期。

### ArpMessage DynamicPct
动态范围百分比，`dynamic_pct`，与你按下时的力度（也就是`velocity`属性）构成了力度自动化的改变范围。

`dynamic_pct`的范围是`0~200`，$velocity \times \frac{dynamic\_pct}{100}$构成了力度改变的一个边界，$velocity$是力度改变的另一个边界。

力度改变的范围就是从这两个里面较小的那一个改变到较大的那一个。