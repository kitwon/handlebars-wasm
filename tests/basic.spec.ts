import Handlebars from '../js';
import { rawStringToArrayBuffer } from '../js/utils';
import { Handlebars as HandlebarsWasm } from '../pkg';

describe('basic context', () => {
  it('most basic', () => {
    expectTemplate('{{foo}}').withInput({ foo: 'foo' }).toCompileTo('foo');
  });

  it('escaping', function () {
    expectTemplate('\\{{foo}}')
      .withInput({ foo: 'food' })
      .toCompileTo('{{foo}}');

    expectTemplate('content \\{{foo}}')
      .withInput({ foo: 'food' })
      .toCompileTo('content {{foo}}');

    expectTemplate('\\\\{{foo}}')
      .withInput({ foo: 'food' })
      .toCompileTo('\\food');

    expectTemplate('content \\\\{{foo}}')
      .withInput({ foo: 'food' })
      .toCompileTo('content \\food');

    expectTemplate('\\\\ {{foo}}')
      .withInput({ foo: 'food' })
      .toCompileTo('\\\\ food');
  });

  it('compiling with a basic context', function () {
    expectTemplate('Goodbye\n{{cruel}}\n{{world}}!')
      .withInput({
        cruel: 'cruel',
        world: 'world'
      })
      .toCompileTo('Goodbye\ncruel\nworld!');
  });

  // Not sure can it compile array context and support dot symbol
  // it("compiling with a string context", function () {
  //   expectTemplate("{{.}}{{length}}").withInput("bye").toCompileTo("bye3");
  // });

  it('compiling with an undefined context', function () {
    expectTemplate('Goodbye\n{{cruel}}\n{{world.bar}}!')
      .withInput(undefined)
      .toCompileTo('Goodbye\n\n!');

    expectTemplate('{{#unless foo}}Goodbye{{../test}}{{test2}}{{/unless}}')
      .withInput(undefined)
      .toCompileTo('Goodbye');
  });

  // TODO: hbs rust should not panic with unregister helper
  it('boolean', function () {
    var string = '{{#goodbye}}GOODBYE {{/goodbye}}cruel {{world}}!';
    expectTemplate(string)
      .withInput({
        goodbye: true,
        world: 'world'
      })
      .withMessage('booleans show the contents when true')
      .toCompileTo('GOODBYE cruel world!');

    expectTemplate(string)
      .withInput({
        goodbye: false,
        world: 'world'
      })
      .withMessage('booleans do not show the contents when false')
      .toCompileTo('cruel world!');
  });

  // TODO: hbs rust support whitespace omit
  // it("comments", function () {
  //   expectTemplate("{{! Goodbye}}Goodbye\n{{cruel}}\n{{world}}!")
  //     .withInput({
  //       cruel: "cruel",
  //       world: "world",
  //     })
  //     .withMessage("comments are ignored")
  //     .toCompileTo("Goodbye\ncruel\nworld!");

  //   // expectTemplate("    {{~! comment ~}}      blah").toCompileTo("blah");

  //   expectTemplate("    {{~!-- long-comment --~}}      blah").toCompileTo(
  //     "blah"
  //   );

  //   expectTemplate("    {{! comment ~}}      blah").toCompileTo("    blah");

  //   expectTemplate("    {{!-- long-comment --~}}      blah").toCompileTo(
  //     "    blah"
  //   );

  //   expectTemplate("    {{~! comment}}      blah").toCompileTo("      blah");

  //   expectTemplate("    {{~!-- long-comment --}}      blah").toCompileTo(
  //     "      blah"
  //   );
  // });
});
