from typing import Generator

def fib(n:int)->Generator[int,None,None]:
    yield 0 #special case
    if n>0: yield 1 #special case
    last: int=0 # initially set to fib(0)
    next: int=1 # initially set to fib(1)
    for _ in range(1,n):
        last, next=next, last+next #variable swap last -> next next -> last_next
        yield next # main generation step

if __name__=="__main__":
    for i in fib(50):
        print(i)
