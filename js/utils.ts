export function rawStringToArrayBuffer(str: string) {
  const arr: number[] = []
  for (let idx = 0; idx < str.length; idx++) {
    arr[idx] = str.charCodeAt(idx) & 0xFF
  }

  return new Uint8Array(arr)
}