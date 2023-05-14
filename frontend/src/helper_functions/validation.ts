import { Action } from "system_types";

export function populateInputVariables(action: Action): string[] {
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

export function populateOutputVariables(action: Action): string[] {
  const input = action.prompt;

  const exampleTagPattern = /\[example\][\s\S]*?\[\/example\]/g;
  const tagPattern = /\[(.*?)\](.*?)\[\/\1\]/g;
      
  // Remove content within [example] tags
  const filteredInput = input.replace(exampleTagPattern, "");
      
  // Find all tags in the remaining text
  const matches = [...filteredInput.matchAll(tagPattern)];
      
  // Extract the tag names from the matches
  const tags = matches.map(match => match[1]);
      
  return tags;
}