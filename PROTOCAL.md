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

## HandShake Message
```
content_bytes: int2
1
name: string
platform: string
```

## Midi Message
```
content_bytes: int2
2
note: int1
velocity: int1
state: int1
```

## Arp Message
```
content_bytes: int2
3
note: int1
velocity: int1
state: int1
method: int1,
rate: int1,
swing_pct: int1,  // 0..=100
up_note_cnt: int1,
velocity_automation: int1,
dynamic_pct: int2, // 0..=200
bpm: int2
```

### method 琶音方法
```
METHOD_NO_METHOD = 0
METHOD_UP = 1
METHOD_DOWN = 2
METHOD_UP_DOWN = 3
METHOD_DOWN_UP = 4
METHOD_3CHORD = 5
METHOD_7CHORD = 6
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

### velocity_automation 力度包络
```
VELOCITY_NO_AUTOMATION = 0
VELOCITY_UP = 1
VELOCITY_DOWN = 2
VELOCITY_UP_DOWN = 3
VELOCITY_DOWN_UP = 4
VELOCITY_STEP = 5
VELOCITY_RANDOM = 6
```

