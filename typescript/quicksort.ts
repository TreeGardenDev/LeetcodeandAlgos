export default function quicksort(arr: number[]):void {

    const pivot = arr.length / 2;

    //TOTALLY WRONG
    //const left: number[] = [];
    //const right: number[] = [];
    //let sortedarr: number[] = [];
    //if (arr.length <= 1) {
    //    return arr;
    //}

    //for (let i = 0; i < arr.length; i++) {
    //    if (arr[pivot] > arr[i]) {
    //        left.push(arr[i]);
    //    }
    //    else {
    //        right.push(arr[i]);
    //    }

    //}
    //console.log(left);
    //console.log(right);
    //const sortedLeft = quicksort(left);
    //const sortedRight = quicksort(right);

    //sortedarr.push(sortedLeft[0], sortedRight[0]);
    //console.log(sortedarr);
    //return sortedarr

    //from primeagen
    qs(arr, 0, arr.length-1);

}
function qs(arr:number[], lo: number, hi:number):void{
    if (lo>=hi){
        return;
    }
    const pivotIndex=partition(arr, lo, hi);

    qs(arr, lo, pivotIndex-1);
    qs(arr, pivotIndex+1, hi);

}
function partition(arr:number[], lo:number, hi:number):number{
    const pivot = arr[hi];

    let idx=lo - 1;

    for (let i=lo; i<hi; ++i){
        if (arr[i]<=pivot){
            idx++;
            const tmp = arr[i];
            arr[idx]=arr[idx];
            arr[idx]=tmp;
        }
    }
     idx++;
    arr[hi]= arr[idx];
    arr[idx]=pivot;

    return idx;

}
const arr = [5, 3, 7, 6, 2, 9];
console.log(arr);
quicksort(arr);
console.log(arr);

