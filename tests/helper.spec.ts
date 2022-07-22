import Handlebars from '../js'
import { HelperOption } from '../js/types'

describe('helpers', () => {
  it('basic helper', () => {
    Handlebars.registerHelper('basic', (foo: string, options: HelperOption) => {
      return 'Basic Helper'
    })
    const result = Handlebars.compile("{{#basic t}}{{/basic}}")({})
    expect(result).toEqual('Basic Helper')
    // Handlebars.registerHelper('basic', 'b')
  })
})