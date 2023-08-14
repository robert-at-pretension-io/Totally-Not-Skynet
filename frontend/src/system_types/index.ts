// import type { Graph } from "graphlib";
import * as t from "io-ts";
import { record } from "io-ts";
import { option } from "io-ts-types";
import { TypeOf } from "io-ts";

export type selectedGraphComponent = {
  type: "Node" | "Edge" | null;
};

export const RuntimeGraphNodeInfo = t.type({
  id: t.string,
  name: t.string
});

export type GraphNodeInfo = TypeOf<typeof RuntimeGraphNodeInfo>;

export const RuntimeEdge = t.type({
  source: RuntimeGraphNodeInfo,
  target: RuntimeGraphNodeInfo
});

export type Edge = TypeOf<typeof RuntimeEdge>;

export const GraphAction = t.partial({
  last_action: t.union([t.literal("add"), t.literal("remove"), t.literal("select"),
  t.literal("deselect"),
  t.literal("reset"),
  t.literal("none")
  ]),
  acted_on: t.union([RuntimeEdge, RuntimeGraphNodeInfo]),
  last_acted_on: t.union([RuntimeEdge, RuntimeGraphNodeInfo])
});

export const RuntimeMongoId = t.type({
  $oid: t.string,
});

export const RuntimePrompt = t.type({
  Prompt: t.type({
    prompt: t.string,
    system: t.string,
  }),
});

export const RuntimeCommand = t.type({
  Command: t.type({
    command: t.string,
  }),
});



export const RuntimeNodeTypeNames = t.keyof({
  "Prompt": null,
  "Process": null,
  "Conditional": null,
  "Command": null
});






export const RuntimeConditional = t.type({
  Conditional: t.type({
    system_variables: t.record(t.string, t.string),
    statement: t.string,
    options: t.record(t.string, t.string), // assuming ObjectId is replaced with string
  }),
});




export const RuntimeGraph = t.type({
  nodes: t.array(RuntimeGraphNodeInfo),
  edges: t.array(RuntimeEdge)
});

export type Graph = TypeOf<typeof RuntimeGraph>;

export const RuntimeProcess = t.type({
  Process: t.type({
    graph: RuntimeGraph,
    initial_variables: t.array(t.string),
    topological_order: t.array(t.string),
  }),
});

export const RuntimeNodeType = t.union([RuntimePrompt, RuntimeProcess, RuntimeConditional, RuntimeCommand]);

export const RuntimeNode = t.type({
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


const RequiredGraph = t.type({
  graph: RuntimeGraph
});



export const RuntimeGraphState = t.intersection([RuntimeGraph, GraphAction]);

export type GraphState = TypeOf<typeof RuntimeGraphState>;

export const RuntimeSystemErrors = t.type({
  name: t.union([t.literal("GraphDoesntExist"), t.literal("GraphStateDoesntExist"), t.literal("OtherError"), t.literal("NodeDoesntExist")])
});

export const RuntimeVerbTypeNames = t.keyof({
  "POST": null,
  "PUT": null,
  "PATCH": null,
  "DELETE": null,
  "GET": null,
});

export const RuntimeExecutionContext = t.type({
  topological_order: t.array(t.string),
  current_node: RuntimeNode, // Use your actual Node type here
  global_variables: record(t.string, t.string),
  execution_id: t.string,
  return_execution_id: option(t.string),
});

export const RuntimeOptionalSystemState = t.partial({
  selected_node: RuntimeNode,
  execution_context: RuntimeExecutionContext,
});

export type OptionalSystemState = TypeOf<typeof RuntimeOptionalSystemState>;

export const RuntimeRequiredSystemState = t.type({
  authenticated: t.boolean,
  websocket_ready: t.boolean,
  graph_state: RuntimeGraphState,
  nodes: t.array(RuntimeNode)
});

export type RequiredSystemState = TypeOf<typeof RuntimeRequiredSystemState>;

export const RuntimeSystemState = t.intersection([RuntimeOptionalSystemState, RuntimeRequiredSystemState]);

export type SystemState = TypeOf<typeof RuntimeSystemState>;

export const RuntimeAuthenticationMessage = t.type({
  AuthenticationMessage: t.type({
    client_email: t.string,
    client_password: t.string
  }),
});

export const RuntimeUserSettings = t.type({
  UserSettings: t.type({
    openai_api_key: t.string,
    mongo_db_uri: t.string,
  }),
});

export const RuntimeCrudBundle = t.type({
  verb: RuntimeVerbTypeNames,
  object: t.union([RuntimeNode, RuntimeAuthenticationMessage, RuntimeUserSettings]),
});

export const RuntimeCommandResponse = t.type({
  error: t.string,
  output: t.string,
});

export const RuntimePromptResponse = t.type({
  response: t.string,
});

export const RuntimeConditionalResponse = t.type({
  chosen_option: t.string,
});

export const RuntimeNodeExecutionResponse = t.union([
  t.type({ Prompt: RuntimePromptResponse }),
  t.type({ Command: RuntimeCommandResponse }),
  t.type({ Conditional: RuntimeConditionalResponse }),
]);

export const RuntimeExecutionResponse = t.type({
  execution_id: t.string,
  container_execution_id: t.union([t.string, t.null]),
  current_node_id: t.string,
  current_node_type: RuntimeNodeTypeNames,
  response: RuntimeNodeExecutionResponse,
});

export const RuntimeResponseObject = t.union([
  RuntimeNode,
  t.literal("InitialMessage"),
  t.literal("UserSettings"),
  t.type({ ExecutionContext: RuntimeExecutionResponse }),
]);

export type NodeTypeNames = TypeOf<typeof RuntimeNodeTypeNames>;
export type SystemErrors = TypeOf<typeof RuntimeSystemErrors>;
export type MongoId = TypeOf<typeof RuntimeMongoId>;
export type Prompt = TypeOf<typeof RuntimePrompt>;
export type Process = TypeOf<typeof RuntimeProcess>;
export type Conditional = TypeOf<typeof RuntimeConditional>;
export type Command = TypeOf<typeof RuntimeCommand>;
export type NodeType = TypeOf<typeof RuntimeNodeType>;
export type Node = TypeOf<typeof RuntimeNode>;
export type CrudBundle = TypeOf<typeof RuntimeCrudBundle>;
export type VerbTypeNames = TypeOf<typeof RuntimeVerbTypeNames>;
export type AuthenticationMessage = TypeOf<typeof RuntimeAuthenticationMessage>;
export type UserSettings = TypeOf<typeof RuntimeUserSettings>;
export type ExecutionContext = TypeOf<typeof RuntimeExecutionContext>;
export type CommandResponse = TypeOf<typeof RuntimeCommandResponse>;
export type PromptResponse = TypeOf<typeof RuntimePromptResponse>;
export type ConditionalResponse = TypeOf<typeof RuntimeConditionalResponse>;
export type NodeExecutionResponse = TypeOf<typeof RuntimeNodeExecutionResponse>;
export type ResponseObject = TypeOf<typeof RuntimeResponseObject>;
export type ExecutionResponse = TypeOf<typeof RuntimeExecutionResponse>;
