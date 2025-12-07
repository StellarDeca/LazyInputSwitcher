#!/bin/bash

# 检查 Fcitx5 框架可用性
command -v fcitx5 >/dev/null 2>&1 && installed_fcitx5=1 || installed_fcitx5=0
command -v fcitx5-remote >/dev/null 2>&1 && installed_remote=1 || installed_remote=0
if [ -f "$HOME/.config/fcitx5/profile" ]; then
    has_config=1
else
    has_config=0
fi

# 存在配置文件并且 Fcitx5 api可用
if [ "$has_config" -eq 1 ] || [ "$installed_fcitx5" -eq 1 ] || [ "$installed_remote" -eq 1 ]; then
    echo "Fcitx5"
else
    echo "None"
fi
