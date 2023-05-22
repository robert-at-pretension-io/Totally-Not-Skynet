import {Graph} from "@dagrejs/graphlib";
import {Process, Action} from "system_types";

// eslint-disable-next-line no-explicit-any
export function isProcess(object: any): object is Process {
  
  console.log("isProcess? ", object)
  
  const debug = true;
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

// eslint-disable-next-line no-explicit-any
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
    _id: {$oid: ""},
    prompt: "",
    input_variables: [],
    output_variables: [],
    name: "",
    system: "",
  };
}

export function newProcess(): Process {
  return {
    _id: {$oid: ""},
    name: "",
    graph: new Graph(),
    description: "",
  };
}
