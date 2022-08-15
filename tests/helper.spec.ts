import Handlebars from "../js";
import { HelperOption } from "../js/types";

describe("helpers", () => {
  it("basic helper", () => {
    Handlebars.registerHelper("basic", (foo: string, options: HelperOption) => {
      return "Basic Helper";
    });
    const result = Handlebars.compile("{{#basic t}}{{/basic}}")({});
    expect(result).toEqual("Basic Helper");
    // Handlebars.registerHelper('basic', 'b')
  });

  it("compiler with js iterator", () => {
    Handlebars.registerHelper(
      "list",
      (list: string[], options: HelperOption) => {
        return list
          .map((i) => {
            // return 1;
            return options.template(i);
          })
          .join("");
      }
    );

    const result = Handlebars.compile(
      "<ul>{{#list people}}<li>1{{this}}</li>{{/list}}</ul>"
    );
    console.log(
      result({
        people: ["Joan", "Kit", "Mary"],
      })
    );
  });
});
