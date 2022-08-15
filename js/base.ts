import { CompileContextFunction, HelperCtxs, HelperOption } from "./types";
import { Handlebars } from "../pkg/handlebars_wasm";
import { rawStringToArrayBuffer } from "./utils";

export default class HandlebarsEnvironment {
  /* Handelbars instance object */
  instance: Handlebars;

  constructor(config: any = {}) {
    this.instance = new Handlebars(config);
  }

  static wrapOptionFn(options: HelperOption, ctxs: HelperCtxs): HelperOption {
    return {
      template: (context) => {
        // const value = rawStringToArrayBuffer(JSON.stringify(context));
        ctxs.template.push(context);
        return options.template(context);
      },
      inverse(context) {
        // const value = rawStringToArrayBuffer(JSON.stringify(context));
        ctxs.inverse.push(context);
        return options.inverse(context);
      },
    };
  }

  /**
   * Register the Handlebars template string
   * and return the function can call with context
   * 
   * eg:
   * ```javascript
   const template = compile("{{foo}}")
   console.log(template({foo: "bar"}))
   * ```
   * 
   * @param template - template string
   * @param options - Handlebars options
   * @returns {CompileContextFunction}
   */
  compile(template: string, options?: any): CompileContextFunction {
    const compiled = (context: Record<string, any>) => {
      // Return compile
      return this.instance.compile(
        template,
        rawStringToArrayBuffer(JSON.stringify(context))
      );
    };

    return compiled;
  }

  /**
   * Registers helpers accessible by any template in the environment.
   *
   * @param name - helper name
   * @param helper - helper function
   */
  registerHelper<A extends (...args: any[]) => any>(name: string, helper: A) {
    /**
     * Wrapper function for js helper
     *
     * @param data context data
     * @param options rust helper option structure
     * @param h helper function
     */
    const wrapper = (data: any, options: HelperOption, h: A) => {
      const ctxs: HelperCtxs = {
        inverse: [],
        template: [],
      };

      const wrapOptionFn = (
        options: HelperOption,
        ctxs: HelperCtxs
      ): HelperOption => {
        return {
          template: (context) => {
            ctxs.template.push(context);
            // return options.template(context);
          },
          inverse(context) {
            ctxs.inverse.push(context);
            // return options.inverse(context);
          },
        };
      };

      let args = [data, wrapOptionFn(options, ctxs)];

      return { text: h(...args), ctxs };
    };

    this.instance.register_helper(name, wrapper, helper);
  }

  /**
   * Unregister a previously registered helper.
   *
   * @param name - helper name
   */
  unregisterHelper(name: string) {}

  /**
   * Registers partials accessible by any template in the environment.
   * @param name - partial name
   * @param partial - partial template string
   */
  registerPartial(name: string, partial: string) {}

  /**
   * Unregister a previously registered helper.
   * @param name - partial name
   */
  unregisterPartial(name: string) {}
}
