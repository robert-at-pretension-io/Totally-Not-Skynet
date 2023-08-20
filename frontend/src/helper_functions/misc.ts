export function areAllPropertiesUndefined<T extends object>(obj: T): boolean {
  return Object.values(obj).every((value) => value === undefined);
}

export function isInstanceOf<T>(obj: any, types: (new () => T)[]): boolean {
  return types.some((type) => obj instanceof type);
}

export function stringToUint8Array(str: string): Uint8Array {
  const utf8Encoder = new TextEncoder();
  return utf8Encoder.encode(str);
}
