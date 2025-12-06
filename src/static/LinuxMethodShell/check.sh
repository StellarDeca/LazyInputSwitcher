#!/bin/bash

# 检查 Fcitx5
if echo "$XMODIFIERS" | grep -i 'fcitx' >/dev/null 2>&1 || [ -n "$FCITX_ENGINE" ]; then
    echo "Fcitx5"
    exit 0

# 检查 Ibus
elif [ -n "$IBUS_ADDRESS" ]; then
    echo "Ibus"
    exit 0

# 其他输入法框架暂时不支持,抛出失败错误码 1
else
    echo "Other"
    exit 1
fi
