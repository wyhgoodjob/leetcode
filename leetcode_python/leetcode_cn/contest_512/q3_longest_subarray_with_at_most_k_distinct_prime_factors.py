from collections import defaultdict
class Solution:
    def longestSubarray(self, nums: list[int], k: int) -> int:
        def build_full_factor_table(max_n:int) -> list[list[int]]:
            is_prime = [True] * (max_n + 1)
            is_prime[0] = is_prime[1] = False
            for i in range(2, int(max_n**0.5) + 1):
                if is_prime[i]:
                    for j in range(i*i, max_n + 1, i):
                        is_prime[j] = False
            result = [[] for _ in range(max_n + 1)]
            for p in range(2, max_n + 1):
                if is_prime[p]:
                    for multiple in range(p, max_n + 1, p):
                        result[multiple].append(p)
            return result
        max_n = max(nums)
        factor_table = build_full_factor_table(max_n)
        left = 0
        ans = 0
        dd_count = defaultdict(int)
        for right in range(len(nums)):
            for i,v in enumerate(factor_table[nums[right]]):
                dd_count[v] += 1
            while len(dd_count) > k and left <= right:
                for i,v in enumerate(factor_table[nums[left]]):
                    dd_count[v] -= 1
                    if dd_count[v] == 0:
                        del dd_count[v]
                left += 1
            ans = max(ans, right - left + 1)
        return ans


if __name__ == "__main__":
    sol = Solution()
    print(sol.longestSubarray([7, 6, 10, 12, 11], 3))