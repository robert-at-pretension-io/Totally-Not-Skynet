import { Edge } from "@dagrejs/graphlib";
import type { Graph } from "graphlib";

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

export type Prompt = {
  _id: MongoId;
  prompt: string;
  input_variables: string[];
  output_variables: string[];
  name: string;
  system: string;
};

export type MongoId = {
  $oid: string;
};

export type Process = {
  _id: MongoId;
  name: string;
  graph: Graph | string; // this is a string when it has been deserialized from the database
  description: string;
  topological_order: string[];
};

export type Message = {
  role: string;
  content: string;
};

export type AiSystemState = {
  actions: Prompt[];
  processes: Process[];
  messages: Message[];
};

export type SystemState = {
  currentlySelected: "action" | "process" | "none";
  websocketReady: boolean;
  selectedAction: Prompt;
  selectedProcess: Process;
  graphState: GraphState;
  websocket: WebSocket;
  executionContext: ExecutionContext;
  aiSystemState: AiSystemState;
};

export type Goal = {
  text: string;
};

export type InitializeProject = {
  initial_message: string;
};

export type OpenaiKey = {
  key: string;
};

export type Prompt = {
  prompt_text: string;
  system: string,
  action_id: string,
};

export type AIResponse = {
  response_text: string;
  action_id: string;
}

export type UpdateAction = {
  action: Prompt;
};

export type UpdateNode = {
  node: AssertsIdentifierTypePredicate;
};

export type MyNode = {
  id: MongoId;
  type_name: string;
  node_content: string;
}

export type CreateAction = {
  create_action: Prompt;
};

export type CreateProcess = {
  create_process: Process;
};

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


export enum NodeType {
  Prompt = "Prompt",
  Process = "Process",
  Command = "Command",
  // variable?
  Conditional = "Conditional"
}