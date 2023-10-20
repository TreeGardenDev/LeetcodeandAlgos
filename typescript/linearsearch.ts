export default function linear_search(haystack: number[], needle: number): boolean{
    for (let i=0; i<haystack.length; i++){
        if (haystack[i]===needle){
            console.log("Found at index: ", i);
            return true;
        }
    }
    return false;
}

function main(){
    let haystack = [1,2,3,4,5,6,7,8,9,10];
    let needle = 5;

    let found = linear_search(haystack, needle);
    console.log(found);
}
main();
