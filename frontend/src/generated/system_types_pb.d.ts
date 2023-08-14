// package: 
// file: system_types.proto

import * as jspb from "google-protobuf";

export class GraphNodeInfo extends jspb.Message {
  getId(): string;
  setId(value: string): void;

  getName(): string;
  setName(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GraphNodeInfo.AsObject;
  static toObject(includeInstance: boolean, msg: GraphNodeInfo): GraphNodeInfo.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GraphNodeInfo, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GraphNodeInfo;
  static deserializeBinaryFromReader(message: GraphNodeInfo, reader: jspb.BinaryReader): GraphNodeInfo;
}

export namespace GraphNodeInfo {
  export type AsObject = {
    id: string,
    name: string,
  }
}

export class Edge extends jspb.Message {
  hasSource(): boolean;
  clearSource(): void;
  getSource(): GraphNodeInfo | undefined;
  setSource(value?: GraphNodeInfo): void;

  hasTarget(): boolean;
  clearTarget(): void;
  getTarget(): GraphNodeInfo | undefined;
  setTarget(value?: GraphNodeInfo): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Edge.AsObject;
  static toObject(includeInstance: boolean, msg: Edge): Edge.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Edge, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Edge;
  static deserializeBinaryFromReader(message: Edge, reader: jspb.BinaryReader): Edge;
}

export namespace Edge {
  export type AsObject = {
    source?: GraphNodeInfo.AsObject,
    target?: GraphNodeInfo.AsObject,
  }
}

export class GraphAction extends jspb.Message {
  getLastAction(): GraphAction.LastActionMap[keyof GraphAction.LastActionMap];
  setLastAction(value: GraphAction.LastActionMap[keyof GraphAction.LastActionMap]): void;

  hasEdge(): boolean;
  clearEdge(): void;
  getEdge(): Edge | undefined;
  setEdge(value?: Edge): void;

  hasNode(): boolean;
  clearNode(): void;
  getNode(): GraphNodeInfo | undefined;
  setNode(value?: GraphNodeInfo): void;

  hasEdgeLast(): boolean;
  clearEdgeLast(): void;
  getEdgeLast(): Edge | undefined;
  setEdgeLast(value?: Edge): void;

  hasNodeLast(): boolean;
  clearNodeLast(): void;
  getNodeLast(): GraphNodeInfo | undefined;
  setNodeLast(value?: GraphNodeInfo): void;

  getActedOnCase(): GraphAction.ActedOnCase;
  getLastActedOnCase(): GraphAction.LastActedOnCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GraphAction.AsObject;
  static toObject(includeInstance: boolean, msg: GraphAction): GraphAction.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GraphAction, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GraphAction;
  static deserializeBinaryFromReader(message: GraphAction, reader: jspb.BinaryReader): GraphAction;
}

export namespace GraphAction {
  export type AsObject = {
    lastAction: GraphAction.LastActionMap[keyof GraphAction.LastActionMap],
    edge?: Edge.AsObject,
    node?: GraphNodeInfo.AsObject,
    edgeLast?: Edge.AsObject,
    nodeLast?: GraphNodeInfo.AsObject,
  }

  export interface LastActionMap {
    ADD: 0;
    REMOVE: 1;
    SELECT: 2;
    DESELECT: 3;
    RESET: 4;
    NONE: 5;
  }

  export const LastAction: LastActionMap;

  export enum ActedOnCase {
    ACTED_ON_NOT_SET = 0,
    EDGE = 2,
    NODE = 3,
  }

  export enum LastActedOnCase {
    LAST_ACTED_ON_NOT_SET = 0,
    EDGE_LAST = 4,
    NODE_LAST = 5,
  }
}

export class MongoId extends jspb.Message {
  getOid(): string;
  setOid(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): MongoId.AsObject;
  static toObject(includeInstance: boolean, msg: MongoId): MongoId.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: MongoId, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): MongoId;
  static deserializeBinaryFromReader(message: MongoId, reader: jspb.BinaryReader): MongoId;
}

export namespace MongoId {
  export type AsObject = {
    oid: string,
  }
}

export class Prompt extends jspb.Message {
  getPrompt(): string;
  setPrompt(value: string): void;

  getSystem(): string;
  setSystem(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Prompt.AsObject;
  static toObject(includeInstance: boolean, msg: Prompt): Prompt.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Prompt, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Prompt;
  static deserializeBinaryFromReader(message: Prompt, reader: jspb.BinaryReader): Prompt;
}

export namespace Prompt {
  export type AsObject = {
    prompt: string,
    system: string,
  }
}

export class Command extends jspb.Message {
  getCommand(): string;
  setCommand(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Command.AsObject;
  static toObject(includeInstance: boolean, msg: Command): Command.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Command, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Command;
  static deserializeBinaryFromReader(message: Command, reader: jspb.BinaryReader): Command;
}

export namespace Command {
  export type AsObject = {
    command: string,
  }
}

export class Conditional extends jspb.Message {
  getSystemVariablesMap(): jspb.Map<string, string>;
  clearSystemVariablesMap(): void;
  getStatement(): string;
  setStatement(value: string): void;

  getOptionsMap(): jspb.Map<string, string>;
  clearOptionsMap(): void;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Conditional.AsObject;
  static toObject(includeInstance: boolean, msg: Conditional): Conditional.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Conditional, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Conditional;
  static deserializeBinaryFromReader(message: Conditional, reader: jspb.BinaryReader): Conditional;
}

export namespace Conditional {
  export type AsObject = {
    systemVariablesMap: Array<[string, string]>,
    statement: string,
    optionsMap: Array<[string, string]>,
  }
}

export class Graph extends jspb.Message {
  clearNodesList(): void;
  getNodesList(): Array<GraphNodeInfo>;
  setNodesList(value: Array<GraphNodeInfo>): void;
  addNodes(value?: GraphNodeInfo, index?: number): GraphNodeInfo;

  clearEdgesList(): void;
  getEdgesList(): Array<Edge>;
  setEdgesList(value: Array<Edge>): void;
  addEdges(value?: Edge, index?: number): Edge;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Graph.AsObject;
  static toObject(includeInstance: boolean, msg: Graph): Graph.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Graph, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Graph;
  static deserializeBinaryFromReader(message: Graph, reader: jspb.BinaryReader): Graph;
}

export namespace Graph {
  export type AsObject = {
    nodesList: Array<GraphNodeInfo.AsObject>,
    edgesList: Array<Edge.AsObject>,
  }
}

export class Process extends jspb.Message {
  hasGraph(): boolean;
  clearGraph(): void;
  getGraph(): Graph | undefined;
  setGraph(value?: Graph): void;

  clearInitialVariablesList(): void;
  getInitialVariablesList(): Array<string>;
  setInitialVariablesList(value: Array<string>): void;
  addInitialVariables(value: string, index?: number): string;

  clearTopologicalOrderList(): void;
  getTopologicalOrderList(): Array<string>;
  setTopologicalOrderList(value: Array<string>): void;
  addTopologicalOrder(value: string, index?: number): string;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Process.AsObject;
  static toObject(includeInstance: boolean, msg: Process): Process.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Process, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Process;
  static deserializeBinaryFromReader(message: Process, reader: jspb.BinaryReader): Process;
}

export namespace Process {
  export type AsObject = {
    graph?: Graph.AsObject,
    initialVariablesList: Array<string>,
    topologicalOrderList: Array<string>,
  }
}

export class Node extends jspb.Message {
  hasId(): boolean;
  clearId(): void;
  getId(): MongoId | undefined;
  setId(value?: MongoId): void;

  getName(): string;
  setName(value: string): void;

  getTypeName(): NodeTypeNamesMap[keyof NodeTypeNamesMap];
  setTypeName(value: NodeTypeNamesMap[keyof NodeTypeNamesMap]): void;

  hasPrompt(): boolean;
  clearPrompt(): void;
  getPrompt(): Prompt | undefined;
  setPrompt(value?: Prompt): void;

  hasProcess(): boolean;
  clearProcess(): void;
  getProcess(): Process | undefined;
  setProcess(value?: Process): void;

  hasConditional(): boolean;
  clearConditional(): void;
  getConditional(): Conditional | undefined;
  setConditional(value?: Conditional): void;

  hasCommand(): boolean;
  clearCommand(): void;
  getCommand(): Command | undefined;
  setCommand(value?: Command): void;

  getDescription(): string;
  setDescription(value: string): void;

  clearInputVariablesList(): void;
  getInputVariablesList(): Array<string>;
  setInputVariablesList(value: Array<string>): void;
  addInputVariables(value: string, index?: number): string;

  clearOutputVariablesList(): void;
  getOutputVariablesList(): Array<string>;
  setOutputVariablesList(value: Array<string>): void;
  addOutputVariables(value: string, index?: number): string;

  getNodeContentCase(): Node.NodeContentCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): Node.AsObject;
  static toObject(includeInstance: boolean, msg: Node): Node.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: Node, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): Node;
  static deserializeBinaryFromReader(message: Node, reader: jspb.BinaryReader): Node;
}

export namespace Node {
  export type AsObject = {
    id?: MongoId.AsObject,
    name: string,
    typeName: NodeTypeNamesMap[keyof NodeTypeNamesMap],
    prompt?: Prompt.AsObject,
    process?: Process.AsObject,
    conditional?: Conditional.AsObject,
    command?: Command.AsObject,
    description: string,
    inputVariablesList: Array<string>,
    outputVariablesList: Array<string>,
  }

  export enum NodeContentCase {
    NODE_CONTENT_NOT_SET = 0,
    PROMPT = 4,
    PROCESS = 5,
    CONDITIONAL = 6,
    COMMAND = 7,
  }
}

export class GraphState extends jspb.Message {
  hasGraph(): boolean;
  clearGraph(): void;
  getGraph(): Graph | undefined;
  setGraph(value?: Graph): void;

  hasAction(): boolean;
  clearAction(): void;
  getAction(): GraphAction | undefined;
  setAction(value?: GraphAction): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): GraphState.AsObject;
  static toObject(includeInstance: boolean, msg: GraphState): GraphState.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: GraphState, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): GraphState;
  static deserializeBinaryFromReader(message: GraphState, reader: jspb.BinaryReader): GraphState;
}

export namespace GraphState {
  export type AsObject = {
    graph?: Graph.AsObject,
    action?: GraphAction.AsObject,
  }
}

export class ExecutionContext extends jspb.Message {
  clearTopologicalOrderList(): void;
  getTopologicalOrderList(): Array<string>;
  setTopologicalOrderList(value: Array<string>): void;
  addTopologicalOrder(value: string, index?: number): string;

  hasCurrentNode(): boolean;
  clearCurrentNode(): void;
  getCurrentNode(): Node | undefined;
  setCurrentNode(value?: Node): void;

  getGlobalVariablesMap(): jspb.Map<string, string>;
  clearGlobalVariablesMap(): void;
  getExecutionId(): string;
  setExecutionId(value: string): void;

  getReturnExecutionId(): string;
  setReturnExecutionId(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ExecutionContext.AsObject;
  static toObject(includeInstance: boolean, msg: ExecutionContext): ExecutionContext.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ExecutionContext, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ExecutionContext;
  static deserializeBinaryFromReader(message: ExecutionContext, reader: jspb.BinaryReader): ExecutionContext;
}

export namespace ExecutionContext {
  export type AsObject = {
    topologicalOrderList: Array<string>,
    currentNode?: Node.AsObject,
    globalVariablesMap: Array<[string, string]>,
    executionId: string,
    returnExecutionId: string,
  }
}

export class SystemState extends jspb.Message {
  getAuthenticated(): boolean;
  setAuthenticated(value: boolean): void;

  getWebsocketReady(): boolean;
  setWebsocketReady(value: boolean): void;

  hasGraphState(): boolean;
  clearGraphState(): void;
  getGraphState(): GraphState | undefined;
  setGraphState(value?: GraphState): void;

  clearNodesList(): void;
  getNodesList(): Array<Node>;
  setNodesList(value: Array<Node>): void;
  addNodes(value?: Node, index?: number): Node;

  hasSelectedNode(): boolean;
  clearSelectedNode(): void;
  getSelectedNode(): Node | undefined;
  setSelectedNode(value?: Node): void;

  hasExecutionContext(): boolean;
  clearExecutionContext(): void;
  getExecutionContext(): ExecutionContext | undefined;
  setExecutionContext(value?: ExecutionContext): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): SystemState.AsObject;
  static toObject(includeInstance: boolean, msg: SystemState): SystemState.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: SystemState, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): SystemState;
  static deserializeBinaryFromReader(message: SystemState, reader: jspb.BinaryReader): SystemState;
}

export namespace SystemState {
  export type AsObject = {
    authenticated: boolean,
    websocketReady: boolean,
    graphState?: GraphState.AsObject,
    nodesList: Array<Node.AsObject>,
    selectedNode?: Node.AsObject,
    executionContext?: ExecutionContext.AsObject,
  }
}

export class AuthenticationMessage extends jspb.Message {
  getClientEmail(): string;
  setClientEmail(value: string): void;

  getClientPassword(): string;
  setClientPassword(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): AuthenticationMessage.AsObject;
  static toObject(includeInstance: boolean, msg: AuthenticationMessage): AuthenticationMessage.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: AuthenticationMessage, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): AuthenticationMessage;
  static deserializeBinaryFromReader(message: AuthenticationMessage, reader: jspb.BinaryReader): AuthenticationMessage;
}

export namespace AuthenticationMessage {
  export type AsObject = {
    clientEmail: string,
    clientPassword: string,
  }
}

export class UserSettings extends jspb.Message {
  getOpenaiApiKey(): string;
  setOpenaiApiKey(value: string): void;

  getMongoDbUri(): string;
  setMongoDbUri(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): UserSettings.AsObject;
  static toObject(includeInstance: boolean, msg: UserSettings): UserSettings.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: UserSettings, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): UserSettings;
  static deserializeBinaryFromReader(message: UserSettings, reader: jspb.BinaryReader): UserSettings;
}

export namespace UserSettings {
  export type AsObject = {
    openaiApiKey: string,
    mongoDbUri: string,
  }
}

export class CrudBundle extends jspb.Message {
  getVerb(): VerbTypeNamesMap[keyof VerbTypeNamesMap];
  setVerb(value: VerbTypeNamesMap[keyof VerbTypeNamesMap]): void;

  hasNode(): boolean;
  clearNode(): void;
  getNode(): Node | undefined;
  setNode(value?: Node): void;

  hasAuthenticationMessage(): boolean;
  clearAuthenticationMessage(): void;
  getAuthenticationMessage(): AuthenticationMessage | undefined;
  setAuthenticationMessage(value?: AuthenticationMessage): void;

  hasUserSettings(): boolean;
  clearUserSettings(): void;
  getUserSettings(): UserSettings | undefined;
  setUserSettings(value?: UserSettings): void;

  getObjectCase(): CrudBundle.ObjectCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CrudBundle.AsObject;
  static toObject(includeInstance: boolean, msg: CrudBundle): CrudBundle.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CrudBundle, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CrudBundle;
  static deserializeBinaryFromReader(message: CrudBundle, reader: jspb.BinaryReader): CrudBundle;
}

export namespace CrudBundle {
  export type AsObject = {
    verb: VerbTypeNamesMap[keyof VerbTypeNamesMap],
    node?: Node.AsObject,
    authenticationMessage?: AuthenticationMessage.AsObject,
    userSettings?: UserSettings.AsObject,
  }

  export enum ObjectCase {
    OBJECT_NOT_SET = 0,
    NODE = 2,
    AUTHENTICATION_MESSAGE = 3,
    USER_SETTINGS = 4,
  }
}

export class CommandResponse extends jspb.Message {
  getError(): string;
  setError(value: string): void;

  getOutput(): string;
  setOutput(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): CommandResponse.AsObject;
  static toObject(includeInstance: boolean, msg: CommandResponse): CommandResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: CommandResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): CommandResponse;
  static deserializeBinaryFromReader(message: CommandResponse, reader: jspb.BinaryReader): CommandResponse;
}

export namespace CommandResponse {
  export type AsObject = {
    error: string,
    output: string,
  }
}

export class PromptResponse extends jspb.Message {
  getResponse(): string;
  setResponse(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): PromptResponse.AsObject;
  static toObject(includeInstance: boolean, msg: PromptResponse): PromptResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: PromptResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): PromptResponse;
  static deserializeBinaryFromReader(message: PromptResponse, reader: jspb.BinaryReader): PromptResponse;
}

export namespace PromptResponse {
  export type AsObject = {
    response: string,
  }
}

export class ConditionalResponse extends jspb.Message {
  getChosenOption(): string;
  setChosenOption(value: string): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ConditionalResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ConditionalResponse): ConditionalResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ConditionalResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ConditionalResponse;
  static deserializeBinaryFromReader(message: ConditionalResponse, reader: jspb.BinaryReader): ConditionalResponse;
}

export namespace ConditionalResponse {
  export type AsObject = {
    chosenOption: string,
  }
}

export class NodeExecutionResponse extends jspb.Message {
  hasPromptResponse(): boolean;
  clearPromptResponse(): void;
  getPromptResponse(): PromptResponse | undefined;
  setPromptResponse(value?: PromptResponse): void;

  hasCommandResponse(): boolean;
  clearCommandResponse(): void;
  getCommandResponse(): CommandResponse | undefined;
  setCommandResponse(value?: CommandResponse): void;

  hasConditionalResponse(): boolean;
  clearConditionalResponse(): void;
  getConditionalResponse(): ConditionalResponse | undefined;
  setConditionalResponse(value?: ConditionalResponse): void;

  getResponseCase(): NodeExecutionResponse.ResponseCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): NodeExecutionResponse.AsObject;
  static toObject(includeInstance: boolean, msg: NodeExecutionResponse): NodeExecutionResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: NodeExecutionResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): NodeExecutionResponse;
  static deserializeBinaryFromReader(message: NodeExecutionResponse, reader: jspb.BinaryReader): NodeExecutionResponse;
}

export namespace NodeExecutionResponse {
  export type AsObject = {
    promptResponse?: PromptResponse.AsObject,
    commandResponse?: CommandResponse.AsObject,
    conditionalResponse?: ConditionalResponse.AsObject,
  }

  export enum ResponseCase {
    RESPONSE_NOT_SET = 0,
    PROMPT_RESPONSE = 1,
    COMMAND_RESPONSE = 2,
    CONDITIONAL_RESPONSE = 3,
  }
}

export class ExecutionResponse extends jspb.Message {
  getExecutionId(): string;
  setExecutionId(value: string): void;

  getContainerExecutionId(): string;
  setContainerExecutionId(value: string): void;

  getCurrentNodeId(): string;
  setCurrentNodeId(value: string): void;

  getCurrentNodeType(): NodeTypeNamesMap[keyof NodeTypeNamesMap];
  setCurrentNodeType(value: NodeTypeNamesMap[keyof NodeTypeNamesMap]): void;

  hasResponse(): boolean;
  clearResponse(): void;
  getResponse(): NodeExecutionResponse | undefined;
  setResponse(value?: NodeExecutionResponse): void;

  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ExecutionResponse.AsObject;
  static toObject(includeInstance: boolean, msg: ExecutionResponse): ExecutionResponse.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ExecutionResponse, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ExecutionResponse;
  static deserializeBinaryFromReader(message: ExecutionResponse, reader: jspb.BinaryReader): ExecutionResponse;
}

export namespace ExecutionResponse {
  export type AsObject = {
    executionId: string,
    containerExecutionId: string,
    currentNodeId: string,
    currentNodeType: NodeTypeNamesMap[keyof NodeTypeNamesMap],
    response?: NodeExecutionResponse.AsObject,
  }
}

export class ResponseObject extends jspb.Message {
  hasNode(): boolean;
  clearNode(): void;
  getNode(): Node | undefined;
  setNode(value?: Node): void;

  hasInitialMessage(): boolean;
  clearInitialMessage(): void;
  getInitialMessage(): string;
  setInitialMessage(value: string): void;

  hasUserSettings(): boolean;
  clearUserSettings(): void;
  getUserSettings(): string;
  setUserSettings(value: string): void;

  hasExecutionResponse(): boolean;
  clearExecutionResponse(): void;
  getExecutionResponse(): ExecutionResponse | undefined;
  setExecutionResponse(value?: ExecutionResponse): void;

  getObjectCase(): ResponseObject.ObjectCase;
  serializeBinary(): Uint8Array;
  toObject(includeInstance?: boolean): ResponseObject.AsObject;
  static toObject(includeInstance: boolean, msg: ResponseObject): ResponseObject.AsObject;
  static extensions: {[key: number]: jspb.ExtensionFieldInfo<jspb.Message>};
  static extensionsBinary: {[key: number]: jspb.ExtensionFieldBinaryInfo<jspb.Message>};
  static serializeBinaryToWriter(message: ResponseObject, writer: jspb.BinaryWriter): void;
  static deserializeBinary(bytes: Uint8Array): ResponseObject;
  static deserializeBinaryFromReader(message: ResponseObject, reader: jspb.BinaryReader): ResponseObject;
}

export namespace ResponseObject {
  export type AsObject = {
    node?: Node.AsObject,
    initialMessage: string,
    userSettings: string,
    executionResponse?: ExecutionResponse.AsObject,
  }

  export enum ObjectCase {
    OBJECT_NOT_SET = 0,
    NODE = 1,
    INITIAL_MESSAGE = 2,
    USER_SETTINGS = 3,
    EXECUTION_RESPONSE = 4,
  }
}

export interface NodeTypeNamesMap {
  PROMPT: 0;
  PROCESS: 1;
  CONDITIONAL: 2;
  COMMAND: 3;
}

export const NodeTypeNames: NodeTypeNamesMap;

export interface SystemErrorsMap {
  GRAPH_DOESNT_EXIST: 0;
  GRAPH_STATE_DOESNT_EXIST: 1;
  OTHER_ERROR: 2;
  NODE_DOESNT_EXIST: 3;
}

export const SystemErrors: SystemErrorsMap;

export interface VerbTypeNamesMap {
  POST: 0;
  PUT: 1;
  PATCH: 2;
  DELETE: 3;
  GET: 4;
}

export const VerbTypeNames: VerbTypeNamesMap;

