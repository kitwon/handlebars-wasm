import { CompileContextFunction, Helper } from "./types"
import { Handlebars } from '../pkg/handlebars_wasm'
import { rawStringToArrayBuffer } from "./utils"

export default class HandlebarsEnvironment {
  /* Handelbars instance object */
  instance: Handlebars

  constructor() {
    this.instance = new Handlebars()
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
  compile(template: string, options: any): CompileContextFunction {
    const compiled = (context: Record<string, any>) => {
      // Return compile
      return this.instance.compile(template, rawStringToArrayBuffer(JSON.stringify(context)))
    }

    return compiled
  }
  
  /**
   * Registers helpers accessible by any template in the environment.
   * 
   * @param name - helper name
   * @param helper - helper function
   */
  registerHelper(name: string, helper: Helper) {
    this.instance.register_helper(name, helper)
  }
  
  /**
   * Unregister a previously registered helper.
   * 
   * @param name - helper name
   */
  unregisterHelper(name: string) {
  }
  
  /**
   * Registers partials accessible by any template in the environment.
   * @param name - partial name
   * @param partial - partial template string
   */
  registerPartial(name: string, partial: string) {
  
  }
  
  /**
   * Unregister a previously registered helper.
   * @param name - partial name
   */
  unresiterPartial(name: string) {
  
  }
}
