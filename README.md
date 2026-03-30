# CPU Monitor

CPU温度和频率监控系统，支持Web界面展示和Docker部署。

## 项目结构

```
cpu-monitor/
├── README.md              # 本文档 - 项目说明
├── record_cpu.sh          # 数据采集脚本
├── view_stats.sh          # 快速查看数据
├── plot_stats.py          # Python绘图工具
├── .gitignore             # Git忽略文件
│
└── server/                # Rust HTTP服务
    ├── src/
    │   ├── main.rs        # Web服务器主程序
    │   └── index.html     # 前端页面(内嵌)
    ├── Cargo.toml         # Rust依赖配置
    ├── Dockerfile         # Docker镜像构建
    ├── deploy.sh          # 一键部署脚本
    ├── collect_stats.sh   # 容器内数据采集(未使用)
    ├── entrypoint.sh      # 容器启动脚本(未使用)
    ├── docker-compose.yml # Docker Compose配置
    ├── index.html         # 前端页面副本
    ├── data/              # CSV数据存储目录
    └── target/            # Rust编译输出
```

## 工作原理

### 数据采集
- **脚本**: `record_cpu.sh`
- **工具**: 
  - `sensors` (lm-sensors) - 读取CPU温度
  - `/sys/devices/system/cpu/cpufreq/policy0/scaling_cur_freq` - 读取CPU频率
- **触发**: Cron每小时执行一次
- **输出**: `~/cpu_monitor/server/data/cpu_stats_YYYYMMDD.csv`

### Web服务 (Rust + Axum)
- **语言**: Rust
- **框架**: Axum 0.8
- **端口**: 容器内3000，外部映射到34567
- **静态编译**: musl target，生成约3MB独立二进制

### 数据格式 (CSV)
```csv
timestamp,cpu_temp_c,cpu_freq_mhz,governor
2026-03-30 07:05:35,72.5,5341,powersave
```

## 快速开始

### 方式一：Docker部署（推荐）

```bash
cd ~/cpu_monitor/server
./deploy.sh
```

访问: http://localhost:34567

### 方式二：直接运行Rust服务

```bash
cd ~/cpu_monitor/server
cargo run --release
```

### 方式三：Python绘图

```bash
# 绘制最近7天数据
~/cpu_monitor/plot_stats.py

# 绘制最近1天数据
~/cpu_monitor/plot_stats.py -d 1

# 分别绘制温度和频率图
~/cpu_monitor/plot_stats.py -s
```

## API端点

| 端点 | 说明 |
|------|------|
| `GET /` | Web界面 |
| `GET /api/current` | 获取最新数据 |
| `GET /api/stats` | 获取历史数据（最近200条）|

## Docker命令

```bash
# 查看日志
docker logs -f cpu-monitor

# 停止/启动
docker stop cpu-monitor
docker start cpu-monitor

# 重启
docker restart cpu-monitor

# 重新部署
cd ~/cpu_monitor/server && ./deploy.sh
```

## 依赖

### 系统依赖
```bash
# Debian/Ubuntu
sudo apt install lm-sensors

# RHEL/CentOS
sudo dnf install lm_sensors

# Arch
sudo pacman -S lm_sensors
```

### Rust编译
```bash
# 安装Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# 添加musl target（静态编译）
rustup target add x86_64-unknown-linux-musl
```

### Python绘图
```bash
pip install pandas matplotlib
```

## 构建流程

1. **Rust编译**: `cargo build --release --target x86_64-unknown-linux-musl`
2. **Docker镜像**: 基于scratch，只复制二进制文件
3. **容器运行**: 挂载数据目录为只读

## 端口说明

- **容器内部**: 3000
- **宿主机**: 34567

## 数据收集频率

- Cron: 每小时执行一次 (`0 * * * *`)
- 位置: `crontab -l`

## 故障排查

### 传感器数据为空
```bash
# 检测传感器
sensors-detect

# 测试
sensors
```

### Docker容器无法访问
```bash
# 检查容器状态
docker ps -a

# 查看日志
docker logs cpu-monitor
```

### 端口冲突
修改 `server/deploy.sh` 中的端口映射 `-p 34567:3000`

## 开发笔记

- 使用Axum 0.8 + Tokio异步运行时
- 前端使用Chart.js绘制双Y轴图表
- 静态编译避免glibc依赖
- scratch镜像最小化体积
