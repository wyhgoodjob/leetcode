"""矩阵技巧：沿对角线蛇形填充 n x n 矩阵。

对应 Rust 侧的 src/basic。运行:
    python -m basic.matrix 3
"""


def generate(n: int) -> list[list[int]]:
    """从右上角开始，沿对角线蛇形填充 n x n 矩阵。"""
    mat = [[0] * n for _ in range(n)]
    num = 1

    # 对角线 i - j = d，d 从 -(n-1) 到 (n-1)
    # 即从右上角那条对角线开始，逐条向左下移动
    for k, d in enumerate(range(-(n - 1), n)):
        lo = max(0, d)                 # 对角线上 i 的下界
        hi = min(n - 1, n - 1 + d)     # 对角线上 i 的上界

        # k 奇 -> 左上到右下(i 递增); k 偶 -> 右下到左上(i 递减)
        rng = range(lo, hi + 1) if k % 2 == 1 else range(hi, lo - 1, -1)

        for i in rng:
            mat[i][i - d] = num        # j = i - d
            num += 1

    return mat


def show(mat: list[list[int]]) -> None:
    """按行打印矩阵。"""
    for row in mat:
        print(" ".join(f"{x:2d}" for x in row))


if __name__ == "__main__":
    import sys

    n = int(sys.argv[1]) if len(sys.argv) > 1 else int(input("please input the number N: "))
    print(f"n = {n}")
    show(generate(n))
