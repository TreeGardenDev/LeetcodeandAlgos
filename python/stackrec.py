def recursivefactorial(n):
    if n==1:
        return 1
    else:
        return n*recursivefactorial(n-1)
factor=recursivefactorial(5)
print(factor)
