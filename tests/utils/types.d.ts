import { TestBench } from "./common";

declare global {
  function expectTemplate(template: string): TestBench;
}
