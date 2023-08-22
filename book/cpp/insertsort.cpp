#include <iostream>
#include <vector>

std::vector<int> insertionsort(std::vector<int> tosort);

int main(){

    std::vector<int> tosort = {12, 5,2,4,11, 6,1,3, 7,13,  8, 9, 10,  15, 14 };
    std::vector<int> sorted = insertionsort(tosort);
    std::cout << "Sorted: "<<std::endl;
    std::cout<<"Size of vector: "<< sorted.size()<<std::endl;
    for (int i=0; i<sorted.size(); i++){
        std::cout<< sorted[i]<<std::endl;
    }

    std::vector<int> correctorder = {1,2,3,4,5,6,7,8,9,10,11,12,13,14,15};

    if (correctorder==sorted){
        std::cout<<"Correct order!"<<std::endl;
    }
    return 0;
}


std::vector<int> insertionsort(std::vector<int> tosort){
    for (int j=1; j<tosort.size(); j++){
        int key=tosort[j];
        int i=j-1;
        while (i>=0 & tosort[i]>key){
            tosort[i+1]=tosort[i];
            i=i-1;
        }
        tosort[i+1]=key;
    }
    return tosort;
}
