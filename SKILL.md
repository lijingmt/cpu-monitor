# Skill: CPU Monitor Project

这是一个完整的CPU温度/频率监控项目，使用Rust编写HTTP服务，Docker部署。

## 项目概述

监控CPU温度和频率，提供：
- 每小时自动数据采集
- Web界面实时查看
- Python脚本生成图表
- Docker一键部署

## 关键文件说明

### 1. record_cpu.sh (数据采集)
```bash
# 路径: ~/cpu_monitor/record_cpu.sh
# 作用: 读取CPU温度和频率，写入CSV
# 工具: sensors命令 + /sys文件系统
# 输出: ~/cpu_monitor/server/data/cpu_stats_YYYYMMDD.csv
# 触发: crontab每小时执行
```

### 2. server/src/main.rs (Rust HTTP服务)
```rust
// 路径: ~/cpu_monitor/server/src/main.rs
// 框架: Axum 0.8
// 路由:
//   GET /           -> 前端页面
//   GET /api/stats  -> 历史数据JSON
//   GET /api/current-> 最新数据JSON
// 端口: 3000
// 数据源: /data目录下的CSV文件
```

### 3. server/src/index.html (前端页面)
```html
<!-- 嵌入到二进制中 -->
<!-- 使用Chart.js绘制双Y轴图表 -->
<!-- 自动30秒刷新 -->
<!-- 显示最近200条记录 -->
```

### 4. server/deploy.sh (一键部署)
```bash
# 功能:
# 1. 编译Rust项目(musl target静态编译)
# 2. 构建Docker镜像(scratch基础镜像)
# 3. 停止旧容器
# 4. 启动新容器(端口34567:3000)
```

### 5. server/Dockerfile
```dockerfile
# 基础镜像: scratch (无任何依赖)
# 复制: 静态编译的二进制文件
# 优势: 镜像仅约3MB
```

## 构建流程图

```
[源码] -> [Rust编译] -> [静态二进制] -> [Docker镜像] -> [容器运行]
   |                                                          |
   |-- src/main.rs                                           |-- 挂载data目录
   |-- src/index.html                                        |-- 端口34567
   |-- Cargo.toml                                            |-- 只读访问
```

## 数据流

```
[sensors命令] -> [record_cpu.sh] -> [CSV文件] -> [Rust读取] -> [JSON API] -> [前端图表]
                    ^                                                   
                    |-- Cron: 0 * * * * (每小时)                        
```

## 快速命令参考

```bash
# === 部署 ===
cd ~/cpu_monitor/server && ./deploy.sh

# === 查看 ===
curl http://localhost:34567/api/current
curl http://localhost:34567/api/stats

# === 日志 ===
docker logs -f cpu-monitor

# === 容器管理 ===
docker stop cpu-monitor
docker start cpu-monitor
docker restart cpu-monitor

# === 绘图 ===
~/cpu_monitor/plot_stats.py -d 7
```

## 修改配置

### 修改端口
编辑 `server/deploy.sh`:
```bash
-p 34567:3000   # 改为 -p 新端口:3000
```

### 修改数据目录
编辑 `record_cpu.sh`:
```bash
DATA_DIR="/your/path/data"
```

### 修改采集频率
```bash
crontab -e
# 改为: */30 * * * * (每30分钟)
# 或: 0 */2 * * * (每2小时)
```

## 故障排查

| 问题 | 原因 | 解决 |
|------|------|------|
| 数据为空 | cron未设置 | 运行 `crontab -l` 检查 |
| 温度显示N/A | sensors未配置 | 运行 `sensors-detect` |
| 容器无法启动 | 端口冲突 | `docker ps` 检查占用 |
| API返回空 | data目录路径错误 | 检查挂载路径 |

## 扩展思路

- 添加告警功能（温度超过阈值通知）
- 支持多CPU核心监控
- 历史数据自动清理
- 导出为Excel格式
