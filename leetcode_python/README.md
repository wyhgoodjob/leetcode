# leetcode_python

个人用 Python 刷 LeetCode 的仓库，目录结构对齐 Rust 侧的 `leetcode_rust/src`。

## 目录

```
leetcode_python/
├── main.py                    # 入口，对应 Rust 的 src/main.rs
├── pyproject.toml             # 对应 Rust 的 Cargo.toml
├── basic/                     # 基础模板 / 技巧，对应 src/basic
│   ├── __init__.py            # 对应 mod.rs，注册模块
│   ├── matrix.py
│   └── ttl_cache.py
├── leetcode_cn/               # 题解，对应 src/leetcode_cn
│   ├── __init__.py
│   ├── p1188_design_bounded_blocking_queue.py
│   └── contest_512/           # 周赛题
│       ├── q2_is_palindromic.py
│       └── q3_longest_subarray_with_at_most_k_distinct_prime_factors.py
└── wip/                       # 未完成 / 有 bug 的草稿
```

## 命名约定

- 题解文件：`p<题号>_<题目名>.py`，如 `p1188_design_bounded_blocking_queue.py`
- 周赛题：放在 `contest_<场次>/q<题号>_<题目名>.py`
- 每个文件用 `if __name__ == "__main__":` 包住自测代码，保证可以被安全 import
- 新模块要登记到对应目录的 `__init__.py`（相当于 Rust 的 `mod.rs`）

## 运行

```bash
cd leetcode_python
python main.py                                  # 入口
python -m basic.matrix 3                        # 运行某个模块
python -m leetcode_cn.p1188_design_bounded_blocking_queue
```

PS: 和 Rust 侧一样，UT / 自测代码基本是 AI 生成的，参考即可。
