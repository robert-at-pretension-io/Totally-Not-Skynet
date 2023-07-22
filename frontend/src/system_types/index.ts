import { Edge } from "@dagrejs/graphlib";
import type { Graph } from "graphlib";
import { Option } from "fp-ts/Option";
import * as t from "io-ts";
import { record } from "io-ts";
import { option } from "io-ts-types";
import { TypeOf } from "io-ts";

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
  selectedNode: Node | null;
  graphState: GraphState;
  websocket: WebSocket;
  executionContext: ExecutionContext;
  nodes: Node[];
};

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
  Node: t.type({
    _id: RuntimeMongoId,
    name: t.string,
    type_name: RuntimeNodeTypeNames,
    node_content: RuntimeNodeType,
    description: t.string,
    input_variables: t.array(t.string),
    output_variables: t.array(t.string),
  })
});

const RuntimeExecutionContext = t.type({
  topological_order: t.array(t.string),
  current_node: RuntimeNode, // Use your actual Node type here
  variables: record(t.string, t.string),
  execution_id: t.string,
  return_execution_id: option(t.string),
});

const RuntimeInitialMessage = t.type({
  InitialMessage: t.type({
    initial_message: t.string,
  }),
});

const RuntimeUserSettings = t.type({
  UserSettings: t.type({
    openai_api_key: t.string,
    mongo_db_uri: t.string,
  }),
});

const RuntimeCrudBundle = t.type({
  verb: RuntimeVerbTypeNames,
  object: t.union([RuntimeNode, RuntimeInitialMessage, RuntimeUserSettings]),
});

const RuntimeCommandResponse = t.type({
  error: t.string,
  output: t.string,
});

const RuntimePromptResponse = t.type({
  response: t.string,
});

const RuntimeConditionalResponse = t.type({
  chosen_option: t.string,
});

const RuntimeNodeExecutionResponse = t.union([
  t.type({ Prompt: RuntimePromptResponse }),
  t.type({ Command: RuntimeCommandResponse }),
  t.type({ Conditional: RuntimeConditionalResponse }),
]);

const RuntimeExecutionResponse = t.type({
  execution_id: t.string,
  container_execution_id: t.union([t.string, t.null]),
  current_node_id: t.string,
  current_node_type: RuntimeNodeTypeNames,
  response: RuntimeNodeExecutionResponse,
});

const RuntimeResponseObject = t.union([
  RuntimeNode,
  t.literal("InitialMessage"),
  t.literal("UserSettings"),
  t.type({ ExecutionContext: RuntimeExecutionResponse }),
]);

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
type ExecutionContext = TypeOf<typeof RuntimeExecutionContext>;
type CommandResponse = TypeOf<typeof RuntimeCommandResponse>;
type PromptResponse = TypeOf<typeof RuntimePromptResponse>;
type ConditionalResponse = TypeOf<typeof RuntimeConditionalResponse>;
type NodeExecutionResponse = TypeOf<typeof RuntimeNodeExecutionResponse>;
type ExecutionResponse = TypeOf<typeof RuntimeExecutionResponse>;
type ResponseObject = TypeOf<typeof RuntimeResponseObject>;

// Export static types
export type { ExecutionContext, CrudBundle, VerbTypeNames, InitialMessage, NodeTypeNames, MongoId, Prompt, NodeType, Node, Process, Conditional, Command, UserSettings, CommandResponse, PromptResponse, ConditionalResponse, NodeExecutionResponse, ExecutionResponse, ResponseObject };

export { RuntimeExecutionContext, RuntimeCrudBundle, RuntimeVerbTypeNames, RuntimeInitialMessage, RuntimeNodeTypeNames, RuntimeMongoId, RuntimePrompt, RuntimeNodeType, RuntimeNode, RuntimeProcess, RuntimeConditional, RuntimeCommand, RuntimeUserSettings, RuntimeCommandResponse, RuntimePromptResponse, RuntimeConditionalResponse, RuntimeNodeExecutionResponse, RuntimeExecutionResponse, RuntimeResponseObject };
