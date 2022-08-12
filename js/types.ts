export type CompileContextFunction = (context: any) => string;

export interface HelperCtxs {
  template: any[];
  inverse: any[];
}

export interface HelperOption {
  template: (context: any) => void;
  inverse: (context: any) => void;
}

export type HelperArguments<T extends any[]> = T extends [...infer Head]
  ? [...Head, HelperOption]
  : any[];

// export type Helper<T extends unknown[]> = (...args: Append<HelperOptions, HelperArguments<T>>) => string
export type Helper<T extends unknown[]> = (
  ...args: HelperArguments<T>
) => string;
