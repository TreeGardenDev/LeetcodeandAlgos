def find_final_factor(number):
    found=False
    originalfactor=0
    while found==False:
        comparelesser=2**originalfactor
        comparegreater=2**(originalfactor+1)
        if comparelesser<=number and number<comparegreater:
            found=True
        else:
            originalfactor=originalfactor+1
    return originalfactor


def deconstruct(number):
    possibilities=[]
    squared=2**number
    possibilities.append(squared) 
    while squared>1:
        squared=squared/2
        possibilities.append(int(squared))
        
    return possibilities
        

def highest_remaining_number(remnum, arrayremains):
    current_highest=0
    index_to_pop=0

    for i in range(0,len(arrayremains)):
        if arrayremains[i]>current_highest and arrayremains[i]<=remnum:
            current_highest=arrayremains[i]
            index_to_pop=i
            current_remaining=remnum-arrayremains[i]
    return current_highest,index_to_pop,current_remaining 


def combine_possibilities(possiblevalues, original):
    solved=False
    odd=False
    sortedarray=sorted(possiblevalues)
    components=[]
    startingcurrent=0
        
    if original%2!=0:
        if len(sortedarray)==1:
            solved=True
        odd=True
        sortedarray.pop(0)
        components.append(1)
        startingcurrent=1

    current=startingcurrent
    remaining=original-current
    rounds=0

    while solved==False:

        current_highest,index_to_pop,remaining=highest_remaining_number(remaining, sortedarray)
        sortedarray.pop(index_to_pop)
        components.append(current_highest)
        if remaining==0:
            solved=True
        
    return components


####TESTS
num=1
factor=find_final_factor(num)
deconstructed=deconstruct(factor)
result=combine_possibilities(deconstructed, num)
print(result)
print("Sum result is: ",sum(result))
assert sum(result)==num

num=1023
factor=find_final_factor(num)
deconstructed=deconstruct(factor)
result=combine_possibilities(deconstructed, num)
print(result)
print("Sum result is: ",sum(result))
assert sum(result)==num

num=74798439
factor=find_final_factor(num)
deconstructed=deconstruct(factor)
result=combine_possibilities(deconstructed, num)
print(result)
print("Sum result is: ",sum(result))
assert sum(result)==num


num=4294967295
factor=find_final_factor(num)
deconstructed=deconstruct(factor)
result=combine_possibilities(deconstructed, num)
print(result)
print("Sum result is: ",sum(result))
assert sum(result)==num
#for i in range(1,  4294967296):
#    num=i
#    print("original is: ", num)
#    factor=find_final_factor(num)
#    deconstructed=deconstruct(factor)
#    result=combine_possibilities(deconstructed, num)
#    print(result)
#    print("Sum result is: ",sum(result))
#    assert sum(result)==num

