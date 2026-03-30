#!/usr/bin/env python3
"""
CPU温度和频率曲线绘图脚本
"""

import pandas as pd
import matplotlib.pyplot as plt
import matplotlib.dates as mdates
from datetime import datetime
import glob
import os
import sys

# 设置中文字体
plt.rcParams['font.sans-serif'] = ['DejaVu Sans', 'Arial', 'Liberation Sans']
plt.rcParams['axes.unicode_minus'] = False

DATA_DIR = os.path.expanduser("~/cpu_monitor/data")
OUTPUT_DIR = os.path.expanduser("~/cpu_monitor/plots")
os.makedirs(OUTPUT_DIR, exist_ok=True)

def load_data(days=7):
    """加载最近N天的数据"""
    all_data = []
    
    # 获取最近的CSV文件
    csv_files = sorted(glob.glob(os.path.join(DATA_DIR, "cpu_stats_*.csv")), reverse=True)[:days]
    
    for f in csv_files:
        try:
            df = pd.read_csv(f, parse_dates=['timestamp'])
            all_data.append(df)
        except Exception as e:
            print(f"读取 {f} 失败: {e}")
    
    if not all_data:
        print("没有找到数据文件")
        return None
    
    return pd.concat(all_data, ignore_index=True).sort_values('timestamp')

def plot_all(df, output_file=None):
    """绘制温度和频率的双Y轴图"""
    if df is None or df.empty:
        print("没有数据可绘制")
        return
    
    fig, ax1 = plt.subplots(figsize=(14, 6))
    
    # 转换温度为数值
    df['cpu_temp_c'] = pd.to_numeric(df['cpu_temp_c'], errors='coerce')
    df['cpu_freq_mhz'] = pd.to_numeric(df['cpu_freq_mhz'], errors='coerce')
    
    x = df['timestamp']
    
    # 左轴：温度
    color1 = '#e74c3c'
    ax1.set_xlabel('Time')
    ax1.set_ylabel('Temperature (°C)', color=color1)
    line1 = ax1.plot(x, df['cpu_temp_c'], color=color1, linewidth=1.5, label='Temperature', alpha=0.8)
    ax1.tick_params(axis='y', labelcolor=color1)
    ax1.grid(True, alpha=0.3)
    
    # 右轴：频率
    ax2 = ax1.twinx()
    color2 = '#3498db'
    ax2.set_ylabel('Frequency (MHz)', color=color2)
    line2 = ax2.plot(x, df['cpu_freq_mhz'], color=color2, linewidth=1.5, label='Frequency', alpha=0.8)
    ax2.tick_params(axis='y', labelcolor=color2)
    
    # 标题
    time_range = f"{x.min().strftime('%m-%d %H:%M')} ~ {x.max().strftime('%m-%d %H:%M')}"
    plt.title(f'CPU Temperature & Frequency\n{time_range}')
    
    # 时间格式
    ax1.xaxis.set_major_formatter(mdates.DateFormatter('%m-%d %H:%M'))
    ax1.xaxis.set_major_locator(mdates.HourLocator(interval=6))
    plt.xticks(rotation=45)
    
    # 图例
    lines = line1 + line2
    labels = [l.get_label() for l in lines]
    ax1.legend(lines, labels, loc='upper left')
    
    plt.tight_layout()
    
    if output_file is None:
        output_file = os.path.join(OUTPUT_DIR, f"cpu_plot_{datetime.now().strftime('%Y%m%d_%H%M%S')}.png")
    
    plt.savefig(output_file, dpi=100)
    print(f"图表已保存: {output_file}")
    plt.close()

def plot_separate(df, output_file=None):
    """分别绘制温度和频率图"""
    if df is None or df.empty:
        print("没有数据可绘制")
        return
    
    df['cpu_temp_c'] = pd.to_numeric(df['cpu_temp_c'], errors='coerce')
    df['cpu_freq_mhz'] = pd.to_numeric(df['cpu_freq_mhz'], errors='coerce')
    
    fig, (ax1, ax2) = plt.subplots(2, 1, figsize=(14, 8))
    x = df['timestamp']
    
    # 温度图
    ax1.plot(x, df['cpu_temp_c'], color='#e74c3c', linewidth=1.5)
    ax1.set_ylabel('Temperature (°C)', color='#e74c3c')
    ax1.set_title('CPU Temperature')
    ax1.grid(True, alpha=0.3)
    ax1.axhline(y=80, color='orange', linestyle='--', alpha=0.5, label='80°C')
    ax1.axhline(y=90, color='red', linestyle='--', alpha=0.5, label='90°C')
    ax1.legend(loc='upper left')
    
    # 频率图
    ax2.plot(x, df['cpu_freq_mhz'], color='#3498db', linewidth=1.5)
    ax2.set_ylabel('Frequency (MHz)', color='#3498db')
    ax2.set_xlabel('Time')
    ax2.set_title('CPU Frequency')
    ax2.grid(True, alpha=0.3)
    
    # 时间格式
    for ax in [ax1, ax2]:
        ax.xaxis.set_major_formatter(mdates.DateFormatter('%m-%d %H:%M'))
        ax.xaxis.set_major_locator(mdates.HourLocator(interval=6))
        plt.sca(ax)
        plt.xticks(rotation=45)
    
    time_range = f"{x.min().strftime('%m-%d')} ~ {x.max().strftime('%m-%d')}"
    fig.suptitle(f'CPU Monitor ({time_range})', fontsize=14)
    
    plt.tight_layout()
    
    if output_file is None:
        output_file = os.path.join(OUTPUT_DIR, f"cpu_plot_sep_{datetime.now().strftime('%Y%m%d_%H%M%S')}.png")
    
    plt.savefig(output_file, dpi=100)
    print(f"图表已保存: {output_file}")
    plt.close()

def main():
    import argparse
    parser = argparse.ArgumentParser(description='CPU监控数据绘图')
    parser.add_argument('-d', '--days', type=int, default=7, help='最近N天的数据 (默认7)')
    parser.add_argument('-s', '--separate', action='store_true', help='分别绘制温度和频率图')
    parser.add_argument('-o', '--output', type=str, help='输出文件路径')
    args = parser.parse_args()
    
    df = load_data(days=args.days)
    if df is not None:
        print(f"加载了 {len(df)} 条记录")
        print(f"时间范围: {df['timestamp'].min()} ~ {df['timestamp'].max()}")
        
        if args.separate:
            plot_separate(df, args.output)
        else:
            plot_all(df, args.output)

if __name__ == '__main__':
    main()
