#!/bin/bash
# Docker内使用的数据收集脚本

DATA_DIR="/app/data"
mkdir -p "$DATA_DIR"

DATA_FILE="$DATA_DIR/cpu_stats_$(date +%Y%m%d).csv"

# 如果文件不存在，创建并写入表头
if [ ! -f "$DATA_FILE" ]; then
    echo "timestamp,cpu_temp_c,cpu_freq_mhz,governor" > "$DATA_FILE"
fi

# 获取CPU温度
CPU_TEMP=$(sensors 2>/dev/null | grep -A 3 "k10temp\|coretemp" | grep -E "Tctl|Package|Core" | head -1 | awk '{print $2}' | tr -d '+°C')

# 备用方法
if [ -z "$CPU_TEMP" ]; then
    for zone in /sys/class/thermal/thermal_zone*; do
        temp=$(cat "$zone/temp" 2>/dev/null)
        if [ -n "$temp" ] && [ "$temp" -gt 1000 ]; then
            CPU_TEMP=$((temp / 1000))
            break
        fi
    done
fi

# 获取CPU频率
FREQ_FILE="/sys/devices/system/cpu/cpufreq/policy0/scaling_cur_freq"
if [ -f "$FREQ_FILE" ]; then
    CPU_FREQ_KHZ=$(cat "$FREQ_FILE")
    CPU_FREQ_MHZ=$((CPU_FREQ_KHZ / 1000))
else
    CPU_FREQ_MHZ="N/A"
fi

# 获取调速器
GOVERNOR=$(cat /sys/devices/system/cpu/cpufreq/policy0/scaling_governor 2>/dev/null || echo "N/A")

# 时间戳
TIMESTAMP=$(date "+%Y-%m-%d %H:%M:%S")

# 写入数据
echo "$TIMESTAMP,$CPU_TEMP,$CPU_FREQ_MHZ,$GOVERNOR" >> "$DATA_FILE"
