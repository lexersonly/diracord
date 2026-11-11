/**
 * Discord Gateway over the global `WebSocket` (Node 22+): identify,
 * heartbeat, resume and reconnect. No compression, no sharding (yet).
 * @module
 */

/** The subset of `WebSocket` used (injectable for tests). */
export interface SocketLike {
  onopen: ((ev: unknown) => void) | null;
  onmessage: ((ev: { data: unknown }) => void) | null;
  onclose: ((ev: { code: number; reason?: string }) => void) | null;
  onerror: ((ev: unknown) => void) | null;
  send(data: string): void;
  close(code?: number): void;
}

/** Creates sockets (defaults to `new WebSocket(url)`). */
export type SocketFactory = (url: string) => SocketLike;

/** Gateway options. */
export interface GatewayOptions {
  token: string;
  intents: number;
  /** URL from `GET /gateway/bot`. */
  url: string;
  onDispatch: (event: string, data: any) => void;
  onLog?: (message: string) => void;
  socket?: SocketFactory;
}

const OP = { DISPATCH: 0, HEARTBEAT: 1, IDENTIFY: 2, RESUME: 6, RECONNECT: 7, INVALID_SESSION: 9, HELLO: 10, ACK: 11 } as const;
/** Close codes after which reconnecting cannot help. */
const FATAL = new Set([4004, 4010, 4011, 4012, 4013, 4014]);

/** One gateway connection that reconnects by itself. */
export class Gateway {
  private readonly opts: GatewayOptions;
  private ws: SocketLike | null = null;
  private seq: number | null = null;
  private sessionId: string | null = null;
  private resumeUrl: string | null = null;
  private heartbeat: ReturnType<typeof setInterval> | null = null;
  private acked = true;
  private stopped = false;

  constructor(opts: GatewayOptions) {
    this.opts = opts;
  }

  /** Opens the connection. */
  connect(): void {
    this.stopped = false;
    const base = this.resumeUrl ?? this.opts.url;
    const factory = this.opts.socket ?? ((u: string) => new WebSocket(u) as unknown as SocketLike);
    const ws = factory(`${base}?v=10&encoding=json`);
    this.ws = ws;
    ws.onmessage = (ev) => this.onMessage(String(ev.data));
    ws.onclose = (ev) => this.onClose(ev.code);
    ws.onerror = () => this.log("erro no websocket");
  }

  /** Closes for good. */
  close(): void {
    this.stopped = true;
    this.stopHeartbeat();
    this.ws?.close(1000);
  }

  private log(m: string): void {
    this.opts.onLog?.(m);
  }

  private send(op: number, d: unknown): void {
    this.ws?.send(JSON.stringify({ op, d }));
  }

  private stopHeartbeat(): void {
    if (this.heartbeat) clearInterval(this.heartbeat);
    this.heartbeat = null;
  }

  private onMessage(raw: string): void {
    let p: { op: number; d: any; s: number | null; t: string | null };
    try {
      p = JSON.parse(raw);
    } catch {
      return;
    }
    if (p.s != null) this.seq = p.s;
    switch (p.op) {
      case OP.HELLO: {
        const every = Number(p.d?.heartbeat_interval ?? 41250);
        this.stopHeartbeat();
        this.acked = true;
        setTimeout(() => this.beat(), Math.floor(every * Math.random()));
        this.heartbeat = setInterval(() => this.beat(), every);
        if (this.sessionId && this.seq != null) {
          this.send(OP.RESUME, { token: this.opts.token, session_id: this.sessionId, seq: this.seq });
        } else {
          this.send(OP.IDENTIFY, {
            token: this.opts.token,
            intents: this.opts.intents,
            properties: { os: process.platform, browser: "diracord", device: "diracord" },
          });
        }
        break;
      }
      case OP.ACK:
        this.acked = true;
        break;
      case OP.HEARTBEAT:
        this.beat(true);
        break;
      case OP.RECONNECT:
        this.ws?.close(4900);
        break;
      case OP.INVALID_SESSION:
        if (!p.d) {
          this.sessionId = null;
          this.seq = null;
          this.resumeUrl = null;
        }
        this.ws?.close(4900);
        break;
      case OP.DISPATCH:
        if (p.t === "READY") {
          this.sessionId = p.d?.session_id ?? null;
          this.resumeUrl = p.d?.resume_gateway_url ?? null;
        }
        if (p.t) this.opts.onDispatch(p.t, p.d);
        break;
    }
  }

  private beat(force = false): void {
    if (!force && !this.acked) {
      this.log("o Discord não respondeu ao heartbeat; reconectando");
      this.ws?.close(4900);
      return;
    }
    this.acked = false;
    this.send(OP.HEARTBEAT, this.seq);
  }

  private onClose(code: number): void {
    this.stopHeartbeat();
    if (this.stopped) return;
    if (FATAL.has(code)) {
      this.log(`conexão recusada pelo Discord (código ${code}). Confira o token e as intents no portal.`);
      return;
    }
    this.log(`conexão caiu (código ${code}); reconectando em 2s`);
    setTimeout(() => this.connect(), 2000);
  }
}
