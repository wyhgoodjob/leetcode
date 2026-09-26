"""基础模板 / 技巧，对应 Rust 侧的 src/basic。

新增模块后把文件名登记到 __all__（相当于 Rust 的 mod.rs）。
这里不 import 子模块，避免 `python -m basic.xxx` 重复导入触发警告。
"""

__all__ = ["matrix", "ttl_cache"]
