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


