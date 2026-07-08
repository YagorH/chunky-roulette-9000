# 🎰 CHUNKY ROULETTE 9000

A lightweight, zero-dependency, step-by-step build randomizer for **The Finals** contestants. Written in pure Rust. Lose your sweat, embrace the pure chaotic fun!

## ✨ Features
* **Zero Dependencies:** No external crates used. Pure, raw standard library execution.
* **Anti-Duplicate Safety:** Dynamic vector management via custom RNG prevents rolled gadgets from repeating.
* **Step-by-Step Intrigue:** Delivers build pieces one-by-one upon hitting Enter for maximum tactical suspense.
* **Informative Stats:** Outputs weapon details including damage, range, and spread.
* **Pure Meme Injection:** Imbued with gaming community inside jokes.

---

## 🚀 Compilation & Running Guide

Ensure you have the Rust toolchain installed. If not, get it from [rustup.rs](https://rustup.rs).

### 🐧 On Linux (Arch / Ubuntu / Debian)

Open your terminal in the repository folder and compile the binary:
```bash
rustc HeavyRand_EN_.rs
```
To execute the generated binary, simply run:
```bash
./HeavyRand_EN_
```

### 🪟 On Windows (CMD / PowerShell)

#### Method A: Compiling natively on Windows
Open your PowerShell or Command Prompt inside the project directory and build the executable:
```powershell
rustc HeavyRand_EN_.rs
```
Run the created `.exe` file:
```powershell
.\HeavyRand_EN_.exe
```
*(Note: A built-in EOF latch is included so the terminal window won't vanish instantly after generation!)*

#### Method B: Cross-compiling from Linux to Windows (MinGW)
If you prefer to bake the `.exe` directly from your Arch Linux system:
1. Install the cross-compilation tools:
   ```bash
   sudo pacman -S mingw-w64-gcc
   rustup target add x86_64-pc-windows-gnu
   ```
2. Build the target executable:
   ```bash
   rustc --target x86_64-pc-windows-gnu HeavyRand_EN_.rs
   ```
Your standalone `HeavyRand_EN_.exe` is ready to be transferred to any Windows machine.

---
## 🎮 Gameplay Verdict
Remember: If you roll a *Sledgehammer* as *Kool-Aid Man* with zero shields — it's not a bad build, it's a test of your inner dominance. See you in the Arena!
