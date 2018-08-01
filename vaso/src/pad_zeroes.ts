export function padZeroes(zeros: number, thing: any) {
    let str = String(thing);
    while (str.length < 2) {
        str = '0' + str;
    }
    return str;
}
