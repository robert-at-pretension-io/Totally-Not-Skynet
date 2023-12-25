import {
  Body,
  Graph,
  Letter,
  SystemState,
  VerbTypes,
  Node,
} from "../generated/system_types";
import { selfIdentify, sendEnvelope } from "./websocket";
import { websocketStore } from "stores/websocketStore";

export function areAllPropertiesUndefined<T extends object>(obj: T): boolean {
  return Object.values(obj).every((value) => value === undefined);
}

export function isInstanceOf<T>(obj: any, types: (new () => T)[]): boolean {
  return types.some((type) => obj instanceof type);
}

export function stringToUint8Array(str: string): Uint8Array {
  const utf8Encoder = new TextEncoder();
  return utf8Encoder.encode(str);
}

export function initializeSystemState(system_state: SystemState): SystemState {
  const graph = new Graph();
  system_state.graph = graph;
  system_state.authenticated = false;
  system_state.websocket_ready = false;
  system_state.local_nodes = [];
  system_state.selected_nodes = [];

  const client_identity = selfIdentify();

  system_state.client_identity = client_identity;

  return system_state;
}

export function getNodes() {
  let websocket: WebSocket;

  websocketStore.subscribe((s) => {
    websocket = s.websocket;
  });

  console.log("requesting nodes from server");
  const letter = new Letter();

  letter.verb = VerbTypes.Get;

  const body = new Body();

  const node = new Node();

  body.node = node;

  letter.body = body;

  sendEnvelope(websocket, [letter]);
}
