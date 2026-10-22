# neo-launcher

High performance desktop GUI for automating workspaces and quickly launching groups of programs. Built for rapid context switching while working or gaming.

Built with a combination of hand-written code and assistance from [ukisai/Swift-Qwen3.8-27B-GGUF](https://huggingface.co/ukisai/Swift-Qwen3.8-27B-GGUF) on an Nvidia RTX 4080.

## Features

Implemented:
- Launch groups of programs from Workspace
- Workspace bookmarks for launching related applications or websites
- Workspace contextual clock
- Configuration via YAML
- Embedded scripts for portability
- Clock with configurable color by time range to reduce time blindness

Planned:
- Configuration via GUI
- Paused execution step when human intervention is required
- Restore window layouts across multiple monitors
- Host specific customization for config portability across systems
- Linux & Mac Support
- OS specific customization for config portability across OS
- Scheduled automations

## Project Goals

1. Learn Rust
2. Application performance is extremely important
3. Experiment with an immediate mode GUI framework while avoiding webviews
4. Speed up feature development by using a locally running LLM
5. Be useful for productivity across multiple desktop applications with multiple work contexts
6. Quickly launch online games with multiple contexts and rapid access to available community tools & sites
