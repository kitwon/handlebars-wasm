import Handlebars from "../js";
import { rawStringToArrayBuffer } from "../js/utils";
import { Handlebars as HandlebarsWasm } from '../pkg'


describe("basic conetxt", () => {
  it("most basic", () => {
    const template = Handlebars.compile("{{foo}}");
    const result = template({ foo: "bar" });

    expect(result).toEqual("bar");
  });

  it("should treat as equal", () => {
    const result = Handlebars.compile("{{foo}}")({foo: "bar"})
    const wasmInst = new HandlebarsWasm()
    const wasm = wasmInst.compile("{{foo}}", rawStringToArrayBuffer(JSON.stringify({foo: "bar"})))

    expect(result).toEqual(wasm)
    expect(result).toEqual("bar")
  })
});
