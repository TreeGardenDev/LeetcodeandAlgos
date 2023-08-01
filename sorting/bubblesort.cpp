#include <iostream>
#include <vector>
using std::vector;

vector<int> bubblesort(vector<int> tobesorted);
vector<int> buildunsorted();

int main(){

    vector<int> tobesorted=buildunsorted();

    vector<int> sorted=bubblesort(tobesorted);

    for (auto &x: sorted){
        std::cout<<"Value: "<<x<<std::endl;
    }

    return 0;
}

vector<int> bubblesort(vector<int> tobesorted){
    
    for (auto &i:tobesorted){
        for (auto &j: tobesorted){
            if (j>i){
                int after=i;
                int before=j;

                j=after;
                i=before;
            }
        }
    }
    return tobesorted;
}

vector<int> buildunsorted(){
    vector<int> unsorted;
    int add;
    std::cout<<"Enter Values For Vector: ";
    while (std::cin>>add){
        std::cout<<"Enter a New Integer Value: ";
        unsorted.push_back(add);
    }
    return unsorted;
}   
