//two crystal balls that will break when dropped
//last point before it will break
//
//ordered array//
//middle jump
//

export default function two_crystal_balls(breaks: boolean[]): number {
    const jmpAmount = Math.floor(Math.sqrt(breaks.length));
    let i = jmpAmount;

    for (; i < breaks.length; i += jmpAmount) {
        if (breaks[i]) {
            break;
        }
    }
    i -= jmpAmount;
    for (let j = 0; j < jmpAmount && i < breaks.length; ++j, ++i) {
        if (breaks[i]) {
            return i;
        }
    }
    return -1;


}

function test() {
    const breaks = [false, false, false, false, false, false, false, false, false, false, false, false, false, false, true];
    console.log(two_crystal_balls(breaks));
    console.log("All tests passed");
}

test();
