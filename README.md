# CPU Monitor

> CPU温度和频率监控系统，Web界面展示，Docker一键部署

[![Rust](https://img.shields.io/badge/Rust-1.94-orange.svg)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/Docker-supported-blue.svg)](https://www.docker.com)
[![License](https://img.shields.io/badge/License-MIT-green.svg)](LICENSE)

## 功能特性

- 实时监控CPU温度和频率
- 每小时自动采集数据
- Web界面可视化展示
- Docker一键部署，镜像仅3MB
- 支持Python生成历史图表

## 快速开始

### 方式一：Docker部署（推荐）

```bash
# 克隆项目
git clone https://github.com/lijingmt/cpu-monitor.git
cd cpu-monitor/server

# 一键部署（编译+构建+运行）
./deploy.sh
```

部署完成后访问：**http://localhost:34567**

### 方式二：直接运行

```bash
cd cpu-monitor/server
cargo build --release
./target/release/cpu_monitor
```

访问：http://localhost:3000

## 项目结构

```
cpu-monitor/
├── README.md              # 本文档
├── record_cpu.sh          # 数据采集脚本（Cron定时执行）
├── view_stats.sh          # 命令行查看数据
├── plot_stats.py          # Python绘图工具
│
└── server/                # Rust HTTP服务
    ├── src/
    │   ├── main.rs        # Web服务器（Axum框架）
    │   └── index.html     # 前端页面
    ├── Cargo.toml         # Rust依赖
    ├── Dockerfile         # Docker镜像定义
    ├── deploy.sh          # 一键部署脚本
    ├── data/              # CSV数据存储
    └── target/            # 编译输出
```

## 工作原理

```
┌─────────────┐     每小时      ┌──────────────┐
│  Cron定时   │ ───────────────> │record_cpu.sh │
└─────────────┘                 └──────────────┘
                                       │
                                       v
                                ┌──────────────┐
                                │ data/*.csv   │
                                └──────────────┘
                                       │
                                       v
┌─────────────┐   HTTP请求   ┌──────────────┐
│  浏览器访问 │ ─────────────> │ Rust服务     │
│  :34567     │ <───────────── │ Axum/3000    │
└─────────────┘   JSON响应    └──────────────┘
```

**数据采集**：`sensors`命令 + `/sys`文件系统读取CPU温度和频率

**Web服务**：Rust + Axum框架，静态编译，独立运行

## Web界面

访问 http://localhost:34567 可以看到：

- 当前温度和频率显示
- 历史曲线图（Chart.js双Y轴图表）
- 每30秒自动刷新
- 显示最近200条记录

## API接口

| 接口 | 方法 | 说明 |
|------|------|------|
| `/` | GET | Web前端页面 |
| `/api/current` | GET | 获取最新数据 |
| `/api/stats` | GET | 获取历史数据（最近200条）|

```bash
# 示例：获取当前数据
curl http://localhost:34567/api/current
# {"timestamp":"2026-03-30 07:11:36","temp":72.0,"freq":5287}
```

## 常用命令

### Docker管理

```bash
# 查看日志
docker logs -f cpu-monitor

# 停止服务
docker stop cpu-monitor

# 启动服务
docker start cpu-monitor

# 重启服务
docker restart cpu-monitor

# 查看状态
docker ps | grep cpu-monitor
```

### 数据查看

```bash
# 命令行查看数据
~/cpu_monitor/view_stats.sh

# Python绘图（最近7天）
~/cpu_monitor/plot_stats.py

# Python绘图（最近1天，分开绘制）
~/cpu_monitor/plot_stats.py -d 1 -s
```

## 系统要求

- Linux系统（支持lm-sensors）
- Docker（推荐）
- 或 Rust 1.70+ (直接运行)
- 或 Python 3.8 + pandas + matplotlib (绘图功能)

### 安装依赖

**lm-sensors（数据采集）：**
```bash
# Debian/Ubuntu
sudo apt install lm-sensors

# RHEL/CentOS/Fedora
sudo dnf install lm_sensors

# Arch Linux
sudo pacman -S lm_sensors
```

**初始化传感器（首次使用）：**
```bash
sudo sensors-detect
# 按提示操作，全部选默认即可
sensors  # 测试是否能读取数据
```

## 配置说明

### 修改端口

编辑 `server/deploy.sh`：
```bash
-p 34567:3000   # 改为 -p 你想要的端口:3000
```

### 修改采集频率

```bash
crontab -e
# 每小时: 0 * * * *
# 每30分钟: */30 * * * *
# 每2小时: 0 */2 * * *
```

### 修改数据存储位置

编辑 `record_cpu.sh`：
```bash
DATA_DIR="/your/custom/path"
```

## 故障排查

| 问题 | 解决方案 |
|------|----------|
| 传感器数据为空 | 运行 `sudo sensors-detect` 检测传感器 |
| 容器无法启动 | `docker logs cpu-monitor` 查看日志 |
| 端口冲突 | 修改 `deploy.sh` 中的端口映射 |
| 权限错误 | 确保脚本有执行权限 `chmod +x *.sh` |

## 开发

```bash
# 克隆项目
git clone https://github.com/lijingmt/cpu-monitor.git
cd cpu-monitor/server

# 安装Rust（如果没有）
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 添加musl target（静态编译）
rustup target add x86_64-unknown-linux-musl

# 开发模式运行
cargo run

# 发布编译
cargo build --release --target x86_64-unknown-linux-musl
```

## License

MIT License

## Star History

如果这个项目对你有帮助，请给个 Star ⭐
