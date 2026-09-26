class Solution:
    def isPalindromic(self, s: str) -> bool:
        def palindrome(s_input: str) -> bool:
            l = 0
            r = len(s_input) - 1
            while l<=r:
                if s_input[l] != s_input[r]:
                    return False
                l+=1
                r-=1
            return True
        judge_str = ''
        for i in range(len(s)):
            part = str(bin(ord(s[i])))[2:]
            print('part:'+part)
            part = ''.join(['0']*(8-len(part))) + part
            print('part:'+part)
            judge_str += part
        return palindrome(judge_str)


if __name__ == "__main__":
    sol = Solution()
    print(sol.isPalindromic("ff"))

