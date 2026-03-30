#!/bin/bash
# 查看CPU统计数据

DATA_DIR="$HOME/cpu_monitor/data"
TODAY=$(date +%Y%m%d)
DATA_FILE="$DATA_DIR/cpu_stats_${TODAY}.csv"

if [ ! -f "$DATA_FILE" ]; then
    echo "今天还没有数据记录"
    exit 1
fi

echo "=== 今天的CPU统计 ==="
echo ""
column -t -s',' "$DATA_FILE"
echo ""
echo "--- 最近10条记录 ---"
tail -10 "$DATA_FILE" | column -t -s','
