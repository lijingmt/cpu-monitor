# CPU Monitor

> CPU温度和频率监控系统，Web界面展示，Docker部署

[![Rust](https://img.shields.io/badge/Rust-1.94-orange.svg)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/Docker-supported-blue.svg)](https://www.docker.com)

## 功能特性

- 实时监控CPU温度和频率
- 每小时自动采集数据
- Web界面可视化展示（支持1/2/3/7/15/30天查询）
- Docker部署，镜像仅3MB
- Python生成历史图表

## 架构说明

本项目采用**分离架构**，数据采集在宿主机，Web展示在Docker容器：

```
┌─────────────────────────────────────────┐
│           宿主机 (Host)                  │
│  ┌────────────────────────────────────┐ │
│  │  1. record_cpu.sh (Cron每小时)      │ │
│  │     ↓                              │ │
│  │  2. sensors命令读取CPU温度         │ │
│  │     ↓                              │ │
│  │  3. 写入 data/*.csv                │ │
│  └────────────────────────────────────┘ │
│           │                              │
│           │ Volume挂载 (只读)             │
└───────────┼──────────────────────────────┘
            │
┌───────────▼──────────────────────────────┐
│        Docker容器                         │
│  ┌────────────────────────────────────┐ │
│  │  Rust Web服务 (Axum)               │ │
│  │     ↓                              │ │
│  │  读取 /data/*.csv                  │ │
│  │     ↓                              │ │
│  │  返回JSON给前端                    │ │
│  └────────────────────────────────────┘ │
└──────────────────────────────────────────┘
```

**为什么这样设计？**
- ✅ 容器不需要特权模式（`--privileged`）
- ✅ 镜像极小（scratch基础镜像，无额外依赖）
- ✅ 更安全（容器只读数据，无法写入）
- ✅ 易维护（容器可以随意重启重建）

## 快速开始

### 1. 安装系统依赖

```bash
# Debian/Ubuntu
sudo apt install lm-sensors

# RHEL/CentOS/Fedora
sudo dnf install lm_sensors

# Arch Linux
sudo pacman -S lm_sensors
```

### 2. 初始化传感器

```bash
sudo sensors-detect
# 按提示操作，全部选默认即可

# 测试
sensors
```

### 3. 部署项目

```bash
# 克隆项目
git clone https://github.com/lijingmt/cpu-monitor.git
cd cpu-monitor/server

# 一键部署（编译+构建+运行）
./deploy.sh
```

部署完成后访问：**http://localhost:34567**

## 详细部署步骤

### 步骤1：配置宿主机采集脚本

```bash
# 复制采集脚本到系统目录
sudo cp record_cpu.sh /usr/local/bin/cpu-monitor-collect
sudo chmod +x /usr/local/bin/cpu-monitor-collect

# 手动测试一次
/usr/local/bin/cpu-monitor-collect
```

### 步骤2：设置定时任务

```bash
# 编辑crontab
crontab -e

# 添加以下行（每小时采集一次）
0 * * * * /usr/local/bin/cpu-monitor-collect >> /var/log/cpu-monitor.log 2>&1
```

### 步骤3：启动Docker服务

```bash
cd cpu-monitor/server
./deploy.sh
```

## 项目结构

```
cpu-monitor/
├── README.md              # 本文档
├── record_cpu.sh          # 数据采集脚本（宿主机运行）
├── view_stats.sh          # 命令行查看数据
├── plot_stats.py          # Python绘图工具
│
└── server/                # Rust HTTP服务（Docker容器）
    ├── src/
    │   ├── main.rs        # Web服务器
    │   └── index.html     # 前端页面
    ├── Cargo.toml         # Rust依赖
    ├── Dockerfile         # Docker镜像
    ├── deploy.sh          # 一键部署脚本
    └── data/              # CSV数据存储（宿主机）
```

## Web界面

访问 http://localhost:34567 可以看到：

- 当前温度和频率显示
- 历史曲线图（Chart.js双Y轴图表）
- 下拉框选择：1/2/3/7/15/30天数据
- 每30秒自动刷新

## API接口

| 接口 | 方法 | 说明 |
|------|------|------|
| `/` | GET | Web前端页面 |
| `/api/current` | GET | 获取最新数据 |
| `/api/stats?days=N` | GET | 获取N天内的数据 |

```bash
# 获取当前数据
curl http://localhost:34567/api/current

# 获取最近7天数据
curl http://localhost:34567/api/stats?days=7
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

### 手动采集数据

```bash
# 立即采集一次
/usr/local/bin/cpu-monitor-collect

# 或使用项目脚本
~/cpu_monitor/record_cpu.sh
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

同时修改 `server/deploy.sh` 中的volume挂载：
```bash
-v "/your/custom/path:/data:ro"
```

## 故障排查

| 问题 | 解决方案 |
|------|----------|
| 传感器数据为空 | 运行 `sudo sensors-detect` 检测传感器 |
| 容器无法启动 | `docker logs cpu-monitor` 查看日志 |
| 端口冲突 | 修改 `deploy.sh` 中的端口映射 |
| 权限错误 | 确保脚本有执行权限 `chmod +x *.sh` |
| 页面没有数据 | 检查 `data/` 目录是否有CSV文件 |
| 下拉框不工作 | 硬刷新浏览器 `Ctrl+Shift+R` |

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
