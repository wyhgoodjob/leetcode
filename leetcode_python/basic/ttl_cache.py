"""TTL 缓存模板：带过期时间的 KV 缓存。

对应 Rust 侧的 src/basic。运行:
    python -m basic.ttl_cache
"""

import time


class TTLCache:
    def __init__(self, timeout: float):
        self.timeout = timeout
        self.cache: dict[str, tuple[str, float]] = {}

    def set(self, k: str, v: str) -> None:
        self.cache[k] = (v, time.monotonic() + self.timeout)

    def get(self, k: str) -> str:
        item = self.cache.get(k)
        if not item:
            return "Not Found"
        v, expired_time = item
        if expired_time < time.monotonic():
            del self.cache[k]
            return "Not Found"
        return v

    def cleanup(self) -> None:
        """惰性清理：删除所有已过期的 key。"""
        now = time.monotonic()
        for k in [key for key, (_, expired_time) in self.cache.items() if expired_time < now]:
            del self.cache[k]


if __name__ == "__main__":
    ttl_cache = TTLCache(5)
    ttl_cache.set("a", "a0")
    print("res:" + ttl_cache.get("a"))
    time.sleep(5)
    print("res:" + ttl_cache.get("a"))
