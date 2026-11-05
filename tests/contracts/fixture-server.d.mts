import type { APIRequest, APIRequestContext, Browser, BrowserContext } from '@playwright/test';
export interface FixtureCredentials { username: string; password: string }
export interface FixtureOptions { binary?: string; webRoot?: string; maxStreams?: number; port?: number; startupTimeoutMs?: number; seed?: boolean; retain?: boolean }
export interface FixtureRequestOptions extends Omit<RequestInit, 'body' | 'headers' | 'method'> {
  principal?: string; method?: string; body?: unknown; headers?: Record<string, string>;
}
export interface FixtureCamera { camera_uuid: string; name: string; [key: string]: unknown }
export interface FixtureUser { id: number; [key: string]: unknown }
export interface ContractFixture {
  directory: string; logPath: string; pid: number; port: number; baseURL: string;
  credentials: Record<string, FixtureCredentials>; users: Record<string, FixtureUser>;
  cameras: Record<string, FixtureCamera>; tokens: Record<string, string>;
  request(route: string, options?: FixtureRequestOptions): Promise<Response>;
  /** Test-only explicit shape assertion. Production adapters must validate unknown JSON. */
  json<T = unknown>(route: string, options?: FixtureRequestOptions): Promise<T>;
  apiContext(request: APIRequest, principal?: string): Promise<APIRequestContext>;
  browserContext(browser: Browser, principal?: string): Promise<BrowserContext>;
  stop(options?: {failed?: boolean}): Promise<void>;
}
export declare const credentials: Readonly<Record<'admin' | 'viewer', FixtureCredentials>>;
export declare function basic(identity: FixtureCredentials): string;
export declare function reservePort(port?: number): Promise<{port: number; release(): Promise<void>}>;
export declare function startFixture(options?: FixtureOptions): Promise<ContractFixture>;
export declare function cameraScope(uuid: string): {type: 'selector'; selector: {version: 1; expression: {op: 'camera_uuid'; values: string[]}}};
export declare function seedFixture(fixture: ContractFixture): Promise<void>;
