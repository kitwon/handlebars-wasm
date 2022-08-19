import { Helper } from "../../js/types";
import Handlebars from "../../js";

export class TestBench {
  template: string;
  helpers: Record<string, Helper<any>>;
  partials: Record<string, any>;
  input?: Record<string, any> | string;
  message: string;
  compileOptions: Record<string, any>;

  constructor(template: string) {
    this.template = template;
    this.helpers = {};
    this.partials = {};
    this.input = {};
    this.compileOptions = {};
    this.message = "";
    this._registerDefaultHelper();
  }

  withInput(input?: TestBench["input"]) {
    this.input = input;
    return this;
  }

  withHelper(name: string, helper: Helper<any>) {
    this.helpers[name] = helper;
    return this;
  }

  withMessage(message: string) {
    this.message = message;
    return this;
  }

  toCompileTo(expectedOutputString: string) {
    expect(this._compileAndExecute()).to.equal(
      expectedOutputString,
      this.message
    );
  }

  _registerDefaultHelper() {
    Handlebars.registerHelper("goodbye", () => {
      return "";
    });
  }

  _compileAndExecute() {
    const template = Handlebars.compile(this.template, this.compileOptions);
    return template(this.input);
  }
}

export function expectTemplate(template: string) {
  return new TestBench(template);
}

global.expectTemplate = expectTemplate;
