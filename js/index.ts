import HandlebarsEnvironment from "./base";

const inst = new HandlebarsEnvironment()

export function create() {
  const env = new HandlebarsEnvironment()
  return env
}

export default inst;