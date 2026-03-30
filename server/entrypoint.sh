#!/bin/bash
set -e

# 启动数据收集（后台）
if [ -f /app/collect_stats.sh ]; then
    echo "Starting data collection..."
    interval=${COLLECT_INTERVAL:-3600}
    while true; do
        /app/collect_stats.sh
        sleep "$interval"
    done &
fi

# 启动web服务
echo "Starting web server on port 3000..."
exec /app/cpu_monitor
