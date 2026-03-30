#!/bin/bash
set -e

echo "=== CPU Monitor Docker Build & Deploy ==="

# 检查docker
if ! command -v docker &> /dev/null; then
    echo "错误: Docker未安装"
    exit 1
fi

# 添加musl target
echo "检查musl target..."
source ~/.cargo/env
rustup target add x86_64-unknown-linux-musl 2>/dev/null || true

# 编译
echo "编译Rust项目..."
cargo build --release --target x86_64-unknown-linux-musl

# 复制index.html
cp src/index.html .

# 停止旧容器
echo "停止旧容器..."
docker stop cpu-monitor 2>/dev/null || true
docker rm cpu-monitor 2>/dev/null || true

# 删除旧镜像
echo "删除旧镜像..."
docker rmi cpu-monitor:latest 2>/dev/null || true

# 构建镜像
echo "构建Docker镜像..."
docker build -t cpu-monitor:latest .

# 创建数据目录
mkdir -p data

# 设置收集脚本到crontab (如果还没设置)
DATA_DIR="$(pwd)/data"
COLLECT_SCRIPT="$HOME/cpu_monitor/record_cpu.sh"

# 更新收集脚本的数据目录
sed -i "s|DATA_DIR=\".*\"|DATA_DIR=\"$DATA_DIR\"|" "$COLLECT_SCRIPT" 2>/dev/null || true

# 检查cron
if ! crontab -l 2>/dev/null | grep -q "record_cpu.sh"; then
    echo "设置每小时数据收集..."
    (crontab -l 2>/dev/null; echo "0 * * * * $COLLECT_SCRIPT >> /tmp/cpu_monitor.log 2>&1") | crontab -
fi

# 运行容器
echo "启动容器..."
docker run -d \
    --name cpu-monitor \
    --restart unless-stopped \
    -p 34567:3000 \
    -v "$(pwd)/data:/data:ro" \
    cpu-monitor:latest

echo ""
echo "=== 部署完成 ==="
echo "访问地址: http://localhost:34567"
echo ""
echo "命令:"
echo "  查看日志: docker logs -f cpu-monitor"
echo "  停止: docker stop cpu-monitor"
echo "  启动: docker start cpu-monitor"
echo "  重启: docker restart cpu-monitor"
echo "  重新部署: ./deploy.sh"
