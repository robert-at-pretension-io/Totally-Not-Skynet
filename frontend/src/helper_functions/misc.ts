import { Process } from "system_types";

export function checkDiff(p1 : Process , p2 : Process) : boolean {
  if (p1.name !== p2.name) {
    return true;
  }
  if (p1.description !== p2.description) {
    return true;
  }
  if (p1.steps.length !== p2.steps.length) {
    return true;
  }
  for (let i = 0; i < p1.steps.length; i++) {
    if (p1.steps[i] !== p2.steps[i]) {
      return true;
    }
  }
  return false;
}