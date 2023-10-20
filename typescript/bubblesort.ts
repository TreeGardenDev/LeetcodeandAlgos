export default function bubble_sort(arr: number[]): number[] {
    for (let i = 0; i < arr.length; i++) {
        for (let j = 0; j < arr.length - i - 1; j++) {
            if (arr[j] > arr[j + 1]) {
                const temp = arr[j];
                arr[j] = arr[j + 1];
                arr[j + 1] = temp;
            }
        }
    }
    return arr
}

function main() {
    let arr = [4, 6, 2, 1, 9, 5, 3, 8, 7]
    let sorted_arr = bubble_sort(arr)
    console.log(sorted_arr)
}

main();
