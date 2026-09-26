from collections import deque
import threading
class BoundedBlockingQueue:
    def __init__(self, capacity:int):
        self.capacity = capacity
        self.q =deque()
        self.lock = threading.Lock()
        self.not_full = threading.Condition(self.lock)
        self.not_empty = threading.Condition(self.lock)

    def put(self, x):
        with self.not_full:
            while len(self.q) == self.capacity():
                self.not_full.wait() # release lock
            self.q.append(x)
            self.not_empty.notify()
    def consume(self):
        with self.not_empty:
            while len(self.q) == 0:
                self.not_empty.wait()
            x = q.pop()
            self.not_full.notify()
            return x

SENTINEL = object()          # 毒丸:通知消费者退出

def producer(q, pid, n):
    for i in range(n):
        q.put(f"P{pid}-{i}")
        print(f"生产者{pid} 放入 P{pid}-{i}")

def consumer(q, cid):
    while True:
        item = q.consume()
        if item is SENTINEL:
            print(f"消费者{cid} 收到结束信号,退出")
            return
        print(f"  消费者{cid} 取出 {item}")

if __name__ == "__main__":
    q = BoundedBlockingQueue(capacity=3)
    prods = [threading.Thread(target=producer, args=(q, i, 5)) for i in range(2)]
    cons  = [threading.Thread(target=consumer, args=(q, i))    for i in range(3)]

    for t in prods + cons: t.start()
    for t in prods: t.join()              # 等生产者都做完
    for _ in cons: q.put(SENTINEL)        # 有几个消费者就发几颗毒丸
    for t in cons: t.join()