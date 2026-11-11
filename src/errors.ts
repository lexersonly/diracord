/** Every error Diracord throws has a line number, so beginners know where to look. */
export class DiracordError extends Error {
  constructor(message: string, public line: number, public col = 0) {
    super(`Line ${line}: ${message}`);
    this.name = "DiracordError";
  }
}
