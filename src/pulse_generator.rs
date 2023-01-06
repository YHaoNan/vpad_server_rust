use std::collections::VecDeque;
use std::ops::{Add, Mul};
use std::pin::Pin;
use std::task::{Context, Poll};
use std::thread;
use std::time::{Duration, SystemTime};

/// 脉冲生成器，最大生成数量 u32::Max，大概是4亿多，超出后自动关闭
/// ticktime中每一个元素代表两次脉冲之间的间隔，第一次脉冲直接触发
///
/// start后，PulseGenerator会进入main loop，以同步方式进行循环，但sleep会让它不过多的消耗CPU时钟周期
/// 对PulseGenerator.next的调用会阻塞底层线程，当到达预设的脉冲时间时，next方法会返回，并携带当前产生脉冲的次数
///
/// 你可以通过调用PulseGenerator.stop来手动停止main loop以释放资源，stop后的PulseGenerator无法重启
/// 但由于在使用for in的迭代器语句中，通常你会将PulseGenerator的所有权移交给迭代器，所以你无法再stop它
/// 所以，PulseGenerator实现了Drop trait，你只需要在适当的情况下break出循环，当PulseGenerator超出作用域
/// stop会自动被调用
pub struct PulseGenerator {
    ticktime: VecDeque<Duration>, // 脉冲间的间隔列表，会从这个列表里循环取间隔
    check_interval: Duration, // 为避免CPU忙碌空转，引入睡眠，该值越大，脉冲生成器精度越低，但CPU负载越低。默认1ms
    first_tick: Duration, // 首次触发延时
    // 下面是用户不关心的辅助属性
    _start_time: SystemTime, // 脉冲生成开始时间
    _is_stopped: bool, // 控制是否停止
    _iter: u32, // 控制迭代次数
    _time_spended: Duration, // 已经花费的时间
}

impl Drop for PulseGenerator {
    fn drop(&mut self) {
        self.stop();
    }
}

impl PulseGenerator {
    pub fn new(ticktime: Vec<Duration>) -> PulseGenerator {
        PulseGenerator::with_check_interval(ticktime, Duration::from_millis(1))
    }
    pub fn with_check_interval(ticktime: Vec<Duration>, check_interval: Duration) -> PulseGenerator {
        if ticktime.len() < 1 {
            panic!("Error when create PulseGenerator. ticktime at least has 1 element.")
        }
        PulseGenerator {
            ticktime: VecDeque::from(ticktime),
            check_interval,
            first_tick: Duration::from_millis(0),
            _start_time: SystemTime::now(),
            _is_stopped: false,
            _iter: 0,
            _time_spended: Duration::from_millis(0),
        }
    }

    pub fn stop(&mut self) {
        self._is_stopped = true;
    }
}

impl Iterator for PulseGenerator {
    type Item = u32; // 迭代次数

    fn next(&mut self) -> Option<Self::Item> {
        while !self._is_stopped {
            let elapsed = self._start_time.elapsed().expect("ERROR WHEN GENERATOR PULSE");
            let i = self._iter;

            let current_duration = if i == 0 {
                self.first_tick
            } else {
                self.ticktime.get((i - 1) as usize % self.ticktime.len()).unwrap().clone()
            };

            if elapsed >= self._time_spended + current_duration {
                if i == u32::MAX {
                    self.stop();
                }
                self._time_spended += current_duration;
                self._iter = i + 1;
                return Some(i)
            }
            thread::sleep(self.check_interval);
        }
        None
    }
}