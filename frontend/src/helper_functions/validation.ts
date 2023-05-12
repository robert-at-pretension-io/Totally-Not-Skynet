import { Action } from "system_types";

export function populateVariables(action: Action): string[] {
  const regex = /\[(.*?)\]/g;
  let match;
  let variables : string []= [];
  while ((match = regex.exec(action.prompt)) !== null) {
    // This is necessary to avoid infinite loops with zero-width matches
    if (match.index === regex.lastIndex) {
      regex.lastIndex++;
    }
  
    // The result can be accessed through the `match`-variable.
    match.forEach((tag, groupIndex) => {
      if (groupIndex === 1) { // Ignore the full match, just add the capture group
        variables.push(tag);
      }
    });
  }
  return variables;
}