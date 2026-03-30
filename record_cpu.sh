#!/bin/bash
DATA_DIR="/home/wapmud/cpu_monitor/server/data"
mkdir -p "$DATA_DIR"
DATA_FILE="$DATA_DIR/cpu_stats_$(date +%Y%m%d).csv"
if [ ! -f "$DATA_FILE" ]; then
    echo "timestamp,cpu_temp_c,cpu_freq_mhz,governor" > "$DATA_FILE"
fi
CPU_TEMP=$(sensors 2>/dev/null | grep -A 3 "k10temp" | grep "Tctl" | awk '{print $2}' | tr -d '+°C')
if [ -z "$CPU_TEMP" ]; then
    TEMP=$(cat /sys/class/thermal/thermal_zone0/temp 2>/dev/null)
    CPU_TEMP=$((TEMP / 1000))
fi
FREQ_FILE="/sys/devices/system/cpu/cpufreq/policy0/scaling_cur_freq"
if [ -f "$FREQ_FILE" ]; then
    CPU_FREQ_MHZ=$(($(cat "$FREQ_FILE") / 1000))
else
    CPU_FREQ_MHZ="N/A"
fi
GOVERNOR=$(cat /sys/devices/system/cpu/cpufreq/policy0/scaling_governor 2>/dev/null || echo "N/A")
TIMESTAMP=$(date "+%Y-%m-%d %H:%M:%S")
echo "$TIMESTAMP,$CPU_TEMP,$CPU_FREQ_MHZ,$GOVERNOR" >> "$DATA_FILE"
echo "[$TIMESTAMP] Temp: ${CPU_TEMP}°C, Freq: ${CPU_FREQ_MHZ}MHz"
