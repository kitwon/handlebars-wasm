export function rawStringToArrayBuffer(str: string) {
  // const arr: number[] = [];
  // if (str) {
  //   for (let idx = 0; idx < str.length; idx++) {
  //     arr[idx] = str.charCodeAt(idx) & 0xff;
  //   }
  // }

  // return new Uint8Array(arr);
  return Buffer.from(str, "utf-8");
}
