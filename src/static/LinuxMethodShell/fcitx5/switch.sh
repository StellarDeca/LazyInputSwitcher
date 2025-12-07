#!/bin/bash

# 设置fcitx5为活动状态并切换当前活动输入法到指定输入法
TARGET_IM="$1"
STATUS=$(fcitx5-remote &> /dev/null)

if [ "$STATUS" -eq 1 ]; then
    fcitx5-remote -o &> /dev/null  # 激活fcitx5
elif [ "$STATUS" -eq 0 ]; then
    fcitx5 &> /dev/null  # 尝试启动fcitx5
fi

fcitx5-remote -s "$TARGET_IM"
