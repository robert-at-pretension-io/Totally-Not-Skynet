import { Edge } from "@dagrejs/graphlib";
import type { Graph } from "graphlib";
import { Option, isSome } from "fp-ts/Option";
import { unsafeCoerce } from 'fp-ts/lib/function';

export type selectedGraphComponent = {
  type: "Node" | "Edge" | null;
};

export type GraphState = {
  graph: Graph;
  lastAction:
  | "addNode"
  | "addEdge"
  | "removeNode"
  | "removeEdge"
  | "selectNode"
  | "deselectNode"
  | "none"
  | "selectEdge"
  | "deselectEdge"
  | "updateNode"
  | "updateEdge"
  | "resetGraph";
  actedOn: Edge | [string, string] | null;
  lastActedOn: Edge | [string, string] | null;
  name: string | null;
  global_variables: Map<string, string>;
};


export type SystemState = {
  websocketReady: boolean;
  selectedNode: Option<Node>;
  graphState: GraphState;
  websocket: WebSocket;
  executionContext: ExecutionContext;
  nodes: Node[];
};


import * as t from "io-ts";
import { option } from "io-ts-types";
import { TypeOf } from "io-ts";

const RuntimeNodeTypeNames = t.keyof({
  "Prompt": null,
  "Process": null,
  "Conditional": null,
  "Command": null
});

const RuntimeVerbTypeNames = t.keyof({
  "POST": null,
  "PUT": null,
  "PATCH": null,
  "DELETE": null,
  "GET": null,
});

const RuntimeMongoId = t.type({
  $oid: t.string,
});

const RuntimePrompt = t.type({
  Prompt: t.type({
    prompt: t.string,
    system: t.string,
  }),
});

const RuntimeProcess = t.type({
  Process: t.type({
    graph: t.string,
    initial_variables: t.array(t.string),
    topological_order: t.array(t.string),
  }),
});

const RuntimeConditional = t.type({
  Conditional: t.type({
    system_variables: t.record(t.string, t.string),
    statement: t.string,
    options: t.record(t.string, t.string), // assuming ObjectId is replaced with string
  }),
});

const RuntimeCommand = t.type({
  Command: t.type({
    command: t.string,
  }),
});

const RuntimeNodeType = t.union([RuntimePrompt, RuntimeProcess, RuntimeConditional, RuntimeCommand]);

const RuntimeNode = t.type({
  _id: RuntimeMongoId,
  name: t.string,
  type_name: RuntimeNodeTypeNames,
  node_content: RuntimeNodeType,
  description: t.string,
  input_variables: t.array(t.string),
  output_variables: t.array(t.string),
});

const RuntimeInitialMessage = t.type({
  initial_message: t.string,
});

const RuntimeUserSettings = t.type({
  openai_api_key: t.string,
  mongo_db_uri: t.string,
});


const RuntimeCrudBundle = t.type({
  verb: RuntimeVerbTypeNames,
  object: t.union([RuntimeNode, RuntimeInitialMessage, RuntimeUserSettings]),
});



type NodeTypeNames = TypeOf<typeof RuntimeNodeTypeNames>;
type MongoId = TypeOf<typeof RuntimeMongoId>;
type Prompt = TypeOf<typeof RuntimePrompt>;
type Process = TypeOf<typeof RuntimeProcess>;
type Conditional = TypeOf<typeof RuntimeConditional>;
type Command = TypeOf<typeof RuntimeCommand>;
type NodeType = TypeOf<typeof RuntimeNodeType>;
type Node = TypeOf<typeof RuntimeNode>;
type CrudBundle = TypeOf<typeof RuntimeCrudBundle>;
type VerbTypeNames = TypeOf<typeof RuntimeVerbTypeNames>;
type InitialMessage = TypeOf<typeof RuntimeInitialMessage>;
type UserSettings = TypeOf<typeof RuntimeUserSettings>;


// Export static types
export type { CrudBundle, VerbTypeNames, InitialMessage, NodeTypeNames, MongoId, Prompt, NodeType, Node, Process, Conditional, Command, UserSettings };

export { RuntimeCrudBundle, RuntimeVerbTypeNames, RuntimeInitialMessage, RuntimeNodeTypeNames, RuntimeMongoId, RuntimePrompt, RuntimeNodeType, RuntimeNode, RuntimeProcess, RuntimeConditional, RuntimeCommand, RuntimeUserSettings };

export type ExecutionContext = {
  local_variables: Map<string, string>;
  global_variables: Map<string, string>;
  current_node: string | null;
  topological_order: string[];
  topological_order_names: (string | undefined)[];
  prompts: Map<string, string>; // map from action id to prompt with filled in variables
  responses: Map<string, string>; // map from action id to response (unparsed)
}
