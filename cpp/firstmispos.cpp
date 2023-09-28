#include <set>
#include <vector>
#include <iostream>

using namespace std;

int firstMissingPositive(vector<int>& nums); 

int main(){
    vector<int> nums = {3,4,-1,1};

    int result= firstMissingPositive(nums);

    cout << result << endl;
    
    return 0;
}

int firstMissingPositive(vector<int>& nums) {
    set<int> data;

    int n=nums.size();

    for (int x:nums){
        if (x>0){
            data.insert(x);
        }
    }
    int i=1;
    while (i<=n){
        if (data.find(i)==data.end()){
            return i;
        }
        i++;
    }
    return i;

}
