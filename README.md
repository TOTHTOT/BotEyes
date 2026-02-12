# BotEyes

Rust 移植的 FluxGarage RoboEyes - 在 OLED 显示屏上绘制流畅动画的机器人眼睛。

原版是 Arduino/Adafruit GFX 实现，现已移植到纯 Rust，支持通用图像生成。

## 功能特性

- **眼睛渲染**：可自定义大小、位置、圆角半径的左右眼睛
- **情绪表达**：默认、疲惫、愤怒、开心
- **眼睛位置**：8 个预定义方向（北、东北、东、东南、南、西南、西、西北）+ 居中
- **动画效果**：眨眼、困惑（水平抖动）、大笑（垂直抖动）、出汗
- **特殊模式**：独眼（单眼）、好奇（眼睛看侧面时变大）
- **帧插值**：使用插值实现平滑动画过渡

## 快速开始

```rust
use boteyes::{RoboEyes, Mood};

// 方式 1：使用默认配置（128x64 OLED，36x36 眼睛）
let mut eyes = RoboEyes::new(128, 64);

// 方式 2：自定义眼睛大小和间距
use boteyes::RoboEyesConfig;
let config = RoboEyesConfig::default()
    .with_eye_width(50)       // 设置眼睛宽度（像素）
    .with_eye_height(50)      // 设置眼睛高度（像素）
    .with_border_radius(12)   // 设置圆角半径（像素）
    .with_space_between(15);  // 设置眼睛间距（像素）

let mut eyes = RoboEyes::new_with_config(128, 64, config);

// 设置心情并睁开眼睛
eyes.set_mood(Mood::Happy);
eyes.open();

// 在当前时间绘制帧（毫秒）
let img = eyes.draw_eyes(1000);

// 保存到文件
img.save("happy_eyes.png")?;
```

## 安装

添加到 `Cargo.toml`：

```toml
[dependencies]
boteyes = { git = "https://github.com/yourusername/BotEyes" }
```

或使用本地路径：

```toml
[dependencies]
boteyes = { path = "path/to/BotEyes" }
```

## 架构设计

```
┌────────────────────────────────────────────────────────────                      用户 API 层─┐
│                               │
│  RoboEyes::new() → 配置 → draw_eyes() → GrayImage          │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                    5 步渲染流水线                              │
│  1. 预计算（插值）                                          │
│  2. 动画处理（宏动画）                                      │
│  3. 形状绘制（眼睛）                                        │
│  4. 情绪覆盖（眼睑）                                        │
│  5. 特效绘制（出汗）                                        │
└─────────────────────────────────────────────────────────────┘
                              ↓
┌─────────────────────────────────────────────────────────────┐
│                      输出层                                  │
│              image::GrayImage（灰度图像）                     │
└─────────────────────────────────────────────────────────────┘
```

## 模块结构

```
src/
├── lib.rs           # RoboEyes 主结构体和 API
├── types/
│   └── mod.rs       # Mood, Position, EyeGeometry, RoboEyesConfig, 配置结构体
├── draw/
│   └── mod.rs       # 绘图原语（圆角矩形、三角形）
└── animation/
    └── mod.rs       # 出汗动画状态
```

## API 参考

### 创建实例

```rust
use boteyes::{RoboEyes, RoboEyesConfig};

// 方式 1：使用默认配置（标准 128x64 OLED 显示屏）
let eyes = RoboEyes::new(128, 64);

// 方式 2：自定义眼睛尺寸和间距
let config = RoboEyesConfig::default()
    .with_eye_width(50)
    .with_eye_height(50)
    .with_border_radius(12)
    .with_space_between(15);

let eyes = RoboEyes::new_with_config(256, 128, config);
```

**默认配置值：**
| 参数 | 默认值 | 说明 |
|------|--------|------|
| `eye_width` | 36 | 眼睛宽度（像素） |
| `eye_height` | 36 | 眼睛高度（像素） |
| `border_radius` | 8 | 圆角半径（像素） |
| `space_between` | 10 | 眼睛间距（像素） |

### 心情设置

```rust
use boteyes::Mood;

// 正常眼睛
eyes.set_mood(Mood::Default);

// 疲惫：上方三角形眼睑
eyes.set_mood(Mood::Tired);

// 愤怒：倾斜的上方眼睑
eyes.set_mood(Mood::Angry);

// 开心：下方圆角遮盖
eyes.set_mood(Mood::Happy);
```

### 眼睛位置

```rust
use boteyes::Position;

eyes.set_position(Position::North);       // 顶部居中
eyes.set_position(Position::NorthEast); // 右上
eyes.set_position(Position::East);       // 右中
eyes.set_position(Position::SouthEast); // 右下
eyes.set_position(Position::South);     // 底部居中
eyes.set_position(Position::SouthWest); // 左下
eyes.set_position(Position::West);      // 左中
eyes.set_position(Position::NorthWest); // 左上
eyes.set_position(Position::Center);    // 居中
```

### 动画

```rust
// 睁开眼睛/闭上眼睛
eyes.open();
eyes.close();
eyes.blink();  // 先闭后开

// 眨左眼
eyes.blink_eyes(true, false);

// 困惑：水平晃动（500ms）
eyes.anim_confused();

// 大笑：垂直弹跳（500ms）
eyes.anim_laugh();

// 额头出汗滴
eyes.set_sweat(true);
```

### 自动动画

```rust
// 自动眨眼（每 3-5 秒）
eyes.set_autoblinker(true, 3, 2);

// 空闲模式：眼睛随机环顾
eyes.set_idle_mode(true, 2, 2);

// 水平闪烁/晃动
eyes.set_h_flicker(true, 3);  // 幅度（像素）

// 垂直闪烁/晃动
eyes.set_v_flicker(true, 5);
```

### 特殊模式

```rust
// 单眼模式
eyes.set_cyclops(true);

// 眼睛看侧面时变大
eyes.set_curiosity(true);
```

### 眼睛几何

```rust
// 设置眼睛尺寸（宽度，高度，单位像素）
eyes.set_size(50, 50);

// 设置圆角半径
eyes.set_border_radius(12, 12);

// 设置眼睛间距（负数表示重叠）
eyes.set_space_between(15);
```

### 两种绘制方式

```rust
// 方式 1：创建新图像（方便但每帧分配内存）
let img = eyes.draw_eyes(time);

// 方式 2：修改现有缓冲区（动画循环推荐，更高效）
let mut buffer = GrayImage::new(128, 64);
loop {
    eyes.draw_into(&mut buffer, time);
    // 使用 buffer...
}
```

## 动画循环示例

```rust
use boteyes::{RoboEyes, Mood};

fn main() {
    let mut eyes = RoboEyes::new(128, 64);
    eyes.set_mood(Mood::Happy);
    eyes.open();

    let mut time = 0u64;

    loop {
        // 在当前时间绘制帧
        let img = eyes.draw_eyes(time);

        // 处理或显示图像...
        // img.save(format!("frame_{}.png", time))?;

        // 推进时间（50 FPS = 每帧 20ms）
        time += 20;
    }
}
```

## 运行示例

```bash
cargo run --example demo
```

这会生成所有心情、位置和动画的截图，保存在 `output/` 目录中。

## 配置

默认值：
- 屏幕尺寸：128x64 像素
- 眼睛尺寸：36x36 像素
- 圆角半径：8 像素
- 眼睛间距：10 像素
- 动画帧率：由调用方控制

## 心情可视化

```
Default:    Tired:       Angry:       Happy:
┌─────┐    ╱  ╲        ╱╲         ─────
│     │   ╱    ╲      ╱  ╲       ═════
│     │  │      │     │    │
└─────┘  ╲    ╱       ╲  ╱
         ╲  ╱         ╲╱
```

## 许可证

本项目是 [FluxGarage RoboEyes](https://github.com/FluxGarage/RoboEyes) 的 Rust 移植版，采用 GPL-3.0 许可证。

## 致谢

- Arduino/Adafruit GFX 原始实现：[FluxGarage RoboEyes](https://github.com/FluxGarage/RoboEyes)，作者 Dennis Hoelscher
- Rust 移植：[Your Name]
