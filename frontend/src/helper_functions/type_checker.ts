import { Process, Action } from "system_types";

export function isProcess(object: any): object is Process {
  let debug = false;
  if (typeof object !== "object") {
    if (debug) {console.log("The object is not an object.");}
    return false;
  }
  
  if (typeof object._id !== "object") {
    if (debug) {console.log("The object does not have an `_id` property.");}
    return false;
  }
  
  if (typeof object.name !== "string") {
    if (debug) {console.log("The object does not have a `name` property.");}
    return false;
  }
  
  if (!Array.isArray(object.steps)) {
    if (debug) {console.log("The object does not have a `steps` property.");}
    return false;
  }
  
  for (const step of object.steps) {
    if (typeof step !== "string") {
      if (debug) {console.log("The `steps` property contains a non-string value.");}
      return false;
    }
  }
  
  if (typeof object.trigger !== "string") {
    if (debug) {console.log("The object does not have a `trigger` property.");}
    return false;
  }
  
  if (typeof object.triggers_next_process !== "string") {
    if (debug) {console.log("The object does not have a `triggers_next_process` property.");}
    return false;
  }
  
  if (typeof object.description !== "string") {
    if (debug) {console.log("The object does not have a `description` property.");}
    return false;
  }
  
  if (typeof object.branch_step !== "string") {
    if (debug) {console.log("The object does not have a `branch_step` property.");}
    return false;
  }
  
  return true;
}
  
export function isAction(object: any): object is Action {
  let debug = false;
  if (typeof object !== "object") {
    if (debug) {console.log("The object is not an object.");}
    return false;
  }
  
  if (typeof object._id !== "object") {
    if (debug) {console.log("The object does not have an `_id` property.");}
    return false;
  }
  
  if (typeof object.prompt !== "string") {
    if (debug) {console.log("The object does not have a `prompt` property.");}
    return false;
  }
  
  if (!Array.isArray(object.input_variables)) {
    if (debug) {console.log("The object does not have a `input_variables` property.");}
    return false;
  }
  
  for (const varItem of object.input_variables) {
    if (typeof varItem !== "string") {
      if (debug) {console.log("The `input_variables` property contains a non-string value.");}
      return false;
    }
  }
  
  if (!Array.isArray(object.output_variables)) {
    if (debug) {console.log("The object does not have a `output_variables` property.");}
    return false;
  }
  
  for (const varItem of object.output_variables) {
    if (typeof varItem !== "string") {
      if (debug) {console.log("The `output_variables` property contains a non-string value.");}
      return false;
    }
  }
  
  if (typeof object.name !== "string") {
    if (debug) {console.log("The object does not have a `name` property.");}
    return false;
  }
  
  if (typeof object.system !== "string") {
    if (debug) {console.log("The object does not have a `system` property.");}
    return false;
  }
  
  return true;
}

export function newAction(): Action {
  return {
    _id: "",
    prompt: "",
    input_variables: [],
    output_variables: [],
    name: "",
    system: ""
  };
}

export function newProcess(): Process {
  return {
    _id: "",
    name: "",
    steps: [],
    trigger: "",
    triggers_next_process: "",
    description: "",
    branch_step: ""
  };
}

export function isNode(object: any): object is Node {
  return typeof object.id === "string" &&
      (typeof object.label === "string" || object.label === undefined) &&
      (isAction(object.data) || object.data === undefined) &&
      (object.type === "action" || object.type === "variable");
}