import { Edge } from "@dagrejs/graphlib";
import type { Graph } from "graphlib";
import { Option } from "fp-ts/Option";

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

export type Message = {
  role: string;
  content: string;
};

export type SystemState = {
  websocketReady: boolean;
  selectedNode: Option<Node>;
  graphState: GraphState;
  websocket: WebSocket;
  executionContext: ExecutionContext;
  nodes: Node[];
};

export type InitializeProject = {
  initial_message: string;
};

export type OpenaiKey = {
  key: string;
};

export type UpdateNode = {
  node: Node;
};

export type CreateNode = {
  node: Node;
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

const RuntimeMongoId = t.type({
  $oid: t.string,
});

const RuntimePrompt = t.type({
  Prompt: t.type({
    prompt: t.string,
    input_variables: t.array(t.string),
    output_variables: t.array(t.string),
    system: t.string,
  }),
});

const RuntimeProcess = t.type({
  Process: t.type({
    graph: t.string,
    description: t.string,
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
});

type NodeTypeNames = TypeOf<typeof RuntimeNodeTypeNames>;
type MongoId = TypeOf<typeof RuntimeMongoId>;
type Prompt = TypeOf<typeof RuntimePrompt>;
type Process = TypeOf<typeof RuntimeProcess>;
type Conditional = TypeOf<typeof RuntimeConditional>;
type Command = TypeOf<typeof RuntimeCommand>;
type NodeType = TypeOf<typeof RuntimeNodeType>;
type Node = TypeOf<typeof RuntimeNode>;

// Export static types
export type { NodeTypeNames, MongoId, Prompt, NodeType, Node, Process, Conditional, Command };

export { RuntimeNodeTypeNames, RuntimeMongoId, RuntimePrompt, RuntimeNodeType, RuntimeNode, RuntimeProcess, RuntimeConditional, RuntimeCommand };

export type ExecutionContext = {
  local_variables: Map<string, string>;
  global_variables: Map<string, string>;
  current_node: string | null;
  topological_order: string[];
  topological_order_names: (string | undefined)[];
  prompts: Map<string, string>; // map from action id to prompt with filled in variables
  responses: Map<string, string>; // map from action id to response (unparsed)
}

export type MessageTypes =
  | { type: "InitializeProject"; data: InitializeProject }
  | { type: "SetOpenAIKey"; data: OpenaiKey } // Replace OpenaiKey with its actual TypeScript type
  | { type: "UpdateNode"; data: UpdateNode }
  | { type: "CreateNode"; data: CreateNode }
  | { type: "Response"; data: Response }
  | { type: "HandleNode"; data: Node };
