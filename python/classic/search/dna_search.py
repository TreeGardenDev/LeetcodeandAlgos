from enum import IntEnum
from typing import Tuple, List

Nucleotide:IntEnum=IntEnum('Nucleotide', ('A','C','G','T'))
lst=[]
lst.append(Nucleotide.A)

Codon=Tuple[Nucleotide, Nucleotide, Nucleotide]

Gene=List[Codon]

def string_to_gene(s:str)->Gene:
    gene:Gene=[]
    for i in range(0, len(s), 3):
        if (i+2) >= len(s):
            return gene
        codon:Codon=(Nucleotide[s[i]], Nucleotide[s[i+1]], Nucleotide[s[i+2]])
        gene.append(codon)
    return gene
def linear_contains(gene: Gene, key:Codon)-> bool:
    for codon in gene:
        if codon == key:
            return True
        return False

def binary_contains(gene:Gene, key:Codon)->bool:
    low:int=0
    high=int(len(gene)-1)
    while low <=high:
        mid:int=(low+high)//2
        if gene[mid] < key:
            low=mid+1
        elif gene[mid] > key:
            high=mid-1
        else:
            return True
    return False
gene_str="ACGTGGCTCTCTAACGTACGTACGTACGGGGTTTATATATACCCTAGGACTCCCTTT"

my_gene=string_to_gene(gene_str)
#print(my_gene)

acg: Codon = (Nucleotide.A, Nucleotide.C, Nucleotide.G)
gat: Codon = (Nucleotide.G, Nucleotide.A, Nucleotide.T)
print(linear_contains(my_gene, acg))  # True
print(linear_contains(my_gene, gat))  # False

my_sorted_gene: Gene = sorted(my_gene)
print(binary_contains(my_sorted_gene, acg))  # True
print(binary_contains(my_sorted_gene, gat))  # False
