#!/usr/bin/env bash
# 第二章 user 目录自检：编译 -> objcopy 出 .bin -> 检查入口地址 -> 用 qemu-riscv64 跑一遍
# 用法（VSCode 的 WSL 终端里执行）：
#   cd ~/test_os/user && bash check-ch2.sh
set -u
cd "$(dirname "$0")" || exit 1

APPS="00hello_world 01store_fault 02power 03priv_inst 04priv_csr"
OUT=target/riscv64gc-unknown-none-elf/release
LOG=build.log
WANT=$((0x80400000))
ROOT=$(pwd)

echo "===== 0. 工具链 ====="
rustc -V || exit 1
cargo -V
rust-objcopy --version >/dev/null 2>&1 || { echo "!! 缺少 rust-objcopy，请先 cargo install cargo-binutils"; exit 1; }
echo

build() {
    # $1 = 额外 RUSTFLAGS（为空则完全按 .cargo/config.toml）
    if [ -n "${1:-}" ]; then
        RUSTFLAGS="$1" cargo build --release > "$LOG" 2>&1
    else
        cargo build --release > "$LOG" 2>&1
    fi
}

# 逐个体检；返回 0=全部合规，1=有问题
inspect() {
    local bad=0 app elf entry size entry_dec
    for app in $APPS; do
        elf="$OUT/$app"
        if [ ! -f "$elf" ]; then
            echo "!! 缺失 $elf"
            return 1
        fi
        rust-objcopy "$elf" --strip-all -O binary "$elf.bin" || return 1
        entry=$(rust-objdump -f "$elf" | awk '/start address/ {print $NF}')
        size=$(stat -c%s "$elf.bin")
        if [ -z "$entry" ]; then
            echo "!! $app 读不到 start address"
            bad=1
            continue
        fi
        entry_dec=$((entry))
        printf '%-16s entry=0x%08x bin=%s bytes\n' "$app" "$entry_dec" "$size"
        if [ "$entry_dec" -ne "$WANT" ]; then
            echo "!! $app 入口应为 0x80400000，实际 0x$(printf '%08x' "$entry_dec")"
            bad=1
        fi
    done
    return $bad
}

echo "===== 1. 编译（按 .cargo/config.toml，链接器 riscv64-unknown-elf-ld / GNU ld）====="
if ! build ""; then
    if grep -q "appears multiple times" "$LOG"; then
        echo "!! 链接脚本被传了两次（上层 .cargo/config.toml 与本目录都写了 -T）"
    fi
    echo "!! 编译失败，最后 40 行日志："
    tail -40 "$LOG"
    exit 1
fi
echo "编译成功"
echo

echo "===== 2. 生成 .bin + 检查入口地址 ====="
if ! inspect; then
    echo
    echo ">> 入口/产物不合规，可能是上层配置里的 -Tsrc/linker.ld 解析到了别的脚本。"
    echo ">> 用绝对路径重新链接一次 ..."
    ABSFLAGS="-Zunstable-options -Clink-arg=-T$ROOT/src/linker.ld -Clinker=riscv64-unknown-elf-ld -Clinker-flavor=gnu -Clink-arg=-s"
    if ! build "$ABSFLAGS"; then
        echo "!! 重新链接仍失败，最后 40 行日志："
        tail -40 "$LOG"
        exit 1
    fi
    inspect || { echo; echo "!! 入口地址仍不符，检查 src/linker.ld 是否以 . = 0x80400000 开头"; exit 1; }
    echo
    echo ">> 已用绝对路径修正，但 .cargo/config.toml 那条 -T 仍然指错，"
    echo ">> 之后 make build 会再次出错，把这段输出发我看一下。"
fi
echo

echo "===== 3. qemu-riscv64 用户态模拟运行 ====="
if command -v qemu-riscv64 >/dev/null 2>&1; then
    # 每个程序最多跑 10 秒：02power 会无限压栈，用户态模拟器里可能卡住
    run() { echo "--- $1"; timeout 10 qemu-riscv64 "$OUT/$1"; echo "exit=$?"; }
    run 00hello_world   # 应打印 Hello, world from user mode program! 后正常退出
    run 01store_fault   # 应触发 Segmentation fault
    run 03priv_inst     # 应触发 Illegal instruction
    run 04priv_csr      # 应触发 Illegal instruction
    echo "--- 02power（书上说明 qemu 下可能不兼容，仅作参考；124=超时被杀，属正常）"
    timeout 10 qemu-riscv64 "$OUT/02power"; echo "exit=$?"
else
    echo "未安装 qemu-riscv64（用户态模拟器），跳过运行验证。想按教程验证可执行："
    echo "  sudo apt update && sudo apt install -y qemu-user"
fi
echo

echo "===== 结论 ====="
echo "OK：5 个用户程序编译通过，入口地址均为 0x80400000"
echo "    .bin 已生成在 $OUT/*.bin"
