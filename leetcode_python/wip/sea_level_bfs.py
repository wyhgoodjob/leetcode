# m*n 
# 数字 -inf ~ inf
# 海平面从0开始涨，每天上涨1
# 第k天陆地面积小于x值，求k
from typing import List
#from collections import deque
class Solution:
    def bfs(grid:List[List[int]], x: int) -> int:
        directions = [(0,1),(0,-1),(-1,0),(1,0)]
        m = len(grid)
        n = len(grid[0])
        cur_count = m*n # 第-1天

        round_min = inf
        for i in range(m):
            round_min = min(grid[i][0], grid[i][-1])
        
        for j in range(n):
            round_min = min(grid[0][j], grid[-1][j])
        
        # 第0天 从最外围开始
        q = []
        for i in range(m):
            

        for j in range(n):
            if grid[i][j] == round_min:
                q.append((i,j))
                cur_count -= 1
        
        cur_day = round_min

        if cur_count < x:
            return cur_day
        
        while q and cur_count >= x:
            next_queue = []
            for _ in range(len(q)):
                cur_i, cur_j = q.pop()
                for d in range(4):
                    next_i = cur_i + directions[d][0]
                    next_j = cur_j + directions[d][1]
                    if 0<=next_i<m and 0<=next_j<n and grid[i][j] <= cur_day:
                        next_queue.append((next_i,next_j))
                        cur_count -= 1
            cur_day += 1
            q = next_queue
        return cur_day - 1

sol = Solution()





