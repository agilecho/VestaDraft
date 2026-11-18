// Generated pure resource contract; no session or host authority.
import type * as Wire from './generated.ts';
import {decodeResource as decode} from '../../../contracts/generated/private-host/resource-codecs.generated.mjs';
export type ResourceRole2 = "OpenBundle" | "InspectionPage" | "HostAck" | "HostNack" | "CommandCompletion" | "FailureNotice";
export interface PayloadByRole {
"OpenBundle": Wire.S14OpenBundle;
"InspectionPage": Wire.S14InspectionPage;
"HostAck": Wire.S14HostAck;
"HostNack": Wire.S14HostNack;
"CommandCompletion": Wire.S14CommandCompletion;
"FailureNotice": Wire.S14FailureNotice;
}
export type ResourceRef<R extends ResourceRole2> = Wire.S14ByteRef & {readonly __role?: R};
export interface DecodeBudget {maxBytes:number;maxDepth:number;maxNodes:number}
export async function decodeResource<R extends ResourceRole2>(role:R, descriptor:ResourceRef<R>, bytes:Uint8Array,budget?:DecodeBudget):Promise<PayloadByRole[R]>{return await decode(role,descriptor,bytes,budget) as PayloadByRole[R];}
export async function readResource<R extends ResourceRole2>(role:R, descriptor:ResourceRef<R>, read:(ref:ResourceRef<R>)=>Promise<Uint8Array>,budget?:DecodeBudget):Promise<PayloadByRole[R]>{return decodeResource(role,descriptor,await read(descriptor),budget);}
