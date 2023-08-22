#include <iostream>
#include <vector>
std::vector<int> insertionsort(std::vector<int> tosort);
std::vector<int> mergesort(std::vector<int>vec1, std::vector<int>vec2);
int main(){
    std::vector<int> vec1 = {2,3,9,10,6,7,8,13};
    std::vector<int> vec2 = {1,10,12,9,11,14,13,15};
    std::vector<int> sortedvec1=insertionsort(vec1);
    std::vector<int> sortedvec2=insertionsort(vec2);

    std::vector<int> sorted = mergesort(sortedvec1, sortedvec2);
    for (int i=0; i<sorted.size(); i++){
        std::cout<< sorted[i]<<std::endl;
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
std::vector<int> mergesort(std::vector<int>vec1, std::vector<int>vec2){
    std::vector<int> sorted;
    for (int i=0; i<vec1.size();){
        for (int j=0; j<vec2.size();){
            if ((i<vec1.size() & vec1[i]<=vec2[j]) | (j==vec2.size())){
               sorted.push_back(vec1[i]);
               i++;
            }
            if ((j<vec2.size() & vec2[j]<vec1[i]) | (i==vec1.size())){
                sorted.push_back(vec2[j]);
                j++;
            }
        }
    }
    return sorted;
}
