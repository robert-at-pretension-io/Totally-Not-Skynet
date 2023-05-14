import { Process, Action } from "system_types";

export function isProcess(object: any): object is Process {
  return typeof object._id === "string" &&
      typeof object.name === "string" &&
      (Array.isArray(object.steps) && object.steps.every((step: string ) => typeof step === "string" )) &&
      typeof object.trigger === "string" &&
      typeof object.triggers_next_process === "string" &&
      typeof object.waits_for_branch_completion === "boolean" &&
      typeof object.description === "string" &&
      typeof object.creates_process_branch === "boolean" &&
      typeof object.branch_step === "string";
}

export function isAction(object: any): object is Action {
  return typeof object._id === "string" &&
      typeof object.prompt === "string" &&
      (Array.isArray(object.input_variables) && object.input_variables.every((varItem: string) => typeof varItem === "string")) &&
      (Array.isArray(object.output_variables) && object.output_variables.every((varItem: string) => typeof varItem === "string")) &&
      typeof object.name === "string" &&
      typeof object.system === "string";
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
    waits_for_branch_completion: false,
    description: "",
    creates_process_branch: false,
    branch_step: ""
  };
}

export function isNode(object: any): object is Node {
    return typeof object.id === 'string' &&
      (typeof object.label === 'string' || object.label === undefined) &&
      (isAction(object.data) || object.data === undefined) &&
      (object.type === 'action' || object.type === 'variable');
  }