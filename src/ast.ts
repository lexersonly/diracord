export type Expr =
  | { t: "num"; v: number; line: number }
  | { t: "bool"; v: boolean; line: number }
  | { t: "null"; line: number }
  | { t: "str"; parts: (string | Expr)[]; line: number }
  | { t: "id"; name: string; line: number }
  | { t: "member"; obj: Expr; prop: string; line: number }
  | { t: "index"; obj: Expr; index: Expr; line: number }
  | { t: "call"; name: string; args: Expr[]; line: number }
  | { t: "list"; items: Expr[]; line: number }
  | { t: "bin"; op: string; l: Expr; r: Expr; line: number }
  | { t: "un"; op: string; e: Expr; line: number };

export type Stmt =
  | { t: "expr"; e: Expr; line: number }
  | { t: "assign"; target: Expr; op: string; value: Expr; line: number }
  | { t: "if"; branches: { cond: Expr; body: Stmt[] }[]; else?: Stmt[]; line: number }
  | { t: "repeat"; count: Expr; body: Stmt[]; line: number }
  | { t: "for"; name: string; iter: Expr; body: Stmt[]; line: number }
  | { t: "stop"; line: number }
  | { t: "return"; value?: Expr; line: number };

export interface Param {
  name: string;
  default?: Expr;
}

export interface Def {
  name: string;
  params: Param[];
  body: Stmt[];
  line: number;
}

export interface Program {
  commands: Map<string, Def>;
  events: Map<string, Def[]>;
  functions: Map<string, Def>;
}
