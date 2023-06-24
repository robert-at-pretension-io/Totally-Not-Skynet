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

type Prompt = {
  prompt: {
    prompt: string;
    input_variables: string[];
    output_variables: string[];
    system: string;
  }

};

export type MongoId = {
  $oid: string;
};

export type Process = {
  process: {
    graph: Graph | string; // this is a string when it has been deserialized from the database
    description: string;
    topological_order: string[];
  }
};

export type Message = {
  role: string;
  content: string;
};


export type SystemState = {
  currentlySelected: "action" | "process" | "none";
  websocketReady: boolean;
  selectedNode: Node;
  graphState: GraphState;
  websocket: WebSocket;
  executionContext: ExecutionContext;
  nodes: Node[];
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

// export type Prompt = {
//   prompt_text: string;
//   system: string,
//   action_id: string,
// };

export type AIResponse = {
  response_text: string;
  action_id: string;
}

export type UpdateAction = {
  action: Prompt;
};

export type UpdateNode = {
  node: Node;
};

export type CreateNode = {
  node: Node;
};

type NodeType = Prompt | Process | Conditional | Command;

type NodeTypeNames = "Prompt" | "Process" | "Conditional" | "Command";

type Conditional = {
  conditional: {
    system_variables: { [key: string]: string };
    statement: string;
    options: { [key: string]: string }; // ObjectId replaced with string
  }
}


type Command = {
  command: {
    command: string;
  }
};



export interface Node {
  _id?: MongoId;
  name: string;
  type_name: NodeTypeNames;
  node_content: NodeType;
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

