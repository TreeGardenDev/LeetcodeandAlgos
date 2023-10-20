export default function binary_search(haystack: number[], needle: number): boolean {
    //ordered array only
    let ret: boolean = false;
    let high: number = haystack.length-1;
    let low: number = 0;

    while (low<high) {
        const middle = Math.floor((low + high) / 2);
        const val = haystack[middle];

        if (needle === val) {
            console.log("found at index: " + middle);
            return true;
        }
        else if (needle < val) {
            high = middle;
        }
        else {
            low = middle + 1;
        }
    }
    return false;


}

// export default function bin_search_good(haystack: number[], needle: number): boolean {
//    let lo = 0;
//    let hi = haystack.length;
//
//    //ordered array only
//    do {
//        const m = Math.floor((lo + hi) / 2);
//        const val = haystack[m];
//        if (val === needle) {
//            return true;
//        }
//        else if (val > needle) {
//            hi = m;
//        }
//        else {
//            lo = m + 1;
//        }
//
//    } while (lo < hi);
//    return false;
//
//}

function main() {
    let haystack: number[] = [1, 2, 3, 4, 5, 6, 7, 8];
    let needle: number = 7;
    let ret: boolean = binary_search(haystack, needle);
    console.log(ret);
    let needle2: number = 9;
    let ret2: boolean = binary_search(haystack, needle2);
    console.log(ret2);
}
main();
