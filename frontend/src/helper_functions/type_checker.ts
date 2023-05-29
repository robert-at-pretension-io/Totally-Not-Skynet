import { Graph } from "@dagrejs/graphlib";
import { Process, Action, AIResponse, GraphState, ExecutionContext } from "system_types";

export function isProcess(object: any): object is Process {

  const debug = false;
  if (typeof object !== "object") {
    if (debug) {
      console.log("The object is not an object.");
    }
    return false;
  }

  //check if object.topological_order is an array of strings
  if (!Array.isArray(object.topological_order)) {
    if (debug) {
      console.log("The object does not have a `topological_order` property.");
    }
    return false;
  }

  if (typeof object._id !== "object") {
    if (debug) {
      console.log("The object does not have an `_id` property.");
    }
    return false;
  }

  if (typeof object.name !== "string") {
    if (debug) {
      console.log("The object does not have a `name` property.");
    }
    return false;
  }

  if (typeof object.graph !== "object") {
    if (debug) {
      console.log("The object does not have a `graph` property.");
    }
    return false;
  }

  if (typeof object.description !== "string") {
    if (debug) {
      console.log("The object does not have a `description` property.");
    }
    return false;
  }

  return true;
}

export function isAction(object: any): object is Action {
  const debug = false;
  if (typeof object !== "object") {
    if (debug) {
      console.log("The object is not an object.");
    }
    return false;
  }

  if (typeof object._id !== "object") {
    if (debug) {
      console.log("The object does not have an `_id` property.");
    }
    return false;
  }

  if (typeof object.prompt !== "string") {
    if (debug) {
      console.log("The object does not have a `prompt` property.");
    }
    return false;
  }

  if (!Array.isArray(object.input_variables)) {
    if (debug) {
      console.log("The object does not have a `input_variables` property.");
    }
    return false;
  }

  for (const varItem of object.input_variables) {
    if (typeof varItem !== "string") {
      if (debug) {
        console.log(
          "The `input_variables` property contains a non-string value."
        );
      }
      return false;
    }
  }

  if (!Array.isArray(object.output_variables)) {
    if (debug) {
      console.log("The object does not have a `output_variables` property.");
    }
    return false;
  }

  for (const varItem of object.output_variables) {
    if (typeof varItem !== "string") {
      if (debug) {
        console.log(
          "The `output_variables` property contains a non-string value."
        );
      }
      return false;
    }
  }

  if (typeof object.name !== "string") {
    if (debug) {
      console.log("The object does not have a `name` property.");
    }
    return false;
  }

  if (typeof object.system !== "string") {
    if (debug) {
      console.log("The object does not have a `system` property.");
    }
    return false;
  }

  return true;
}

export function newAction(): Action {
  return {
    _id: { $oid: "" },
    prompt: "",
    input_variables: [],
    output_variables: [],
    name: "",
    system: "",
  };
}

export function isResponse(object: any): object is AIResponse {
  if (object === null || object === undefined) {
    return false;
  }
  else {
    return object && "response_text" in object && "action_id" in object;

  }
}

export function newProcess(): Process {
  return {
    _id: { $oid: "" },
    name: "",
    graph: new Graph(),
    description: "",
    topological_order: [],
  };
}

export function newGraphState(): GraphState {
  return {
    graph: new Graph(), // replace with correct way to create a new Graph object
    lastAction: "none",
    actedOn: null,
    lastActedOn: null,
    name: null,
    global_variables: new Map<string, string>(),
  };
}

export function NewExecutionContext(): ExecutionContext {
  return {
    local_variables: new Map<string, string>(),
    global_variables: new Map<string, string>(),
    topological_order: [],
    topological_order_names: [],
    current_node: null,
    prompts: new Map<string, string>(),
    responses: new Map<string, string>(),
  };
}