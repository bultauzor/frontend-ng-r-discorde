export class WsCommand {
  from: string
  message: { timestamp: number, author: string, message: string }


  constructor(from: string, message: { timestamp: number; author: string; message: string }) {
    this.from = from;
    this.message = message;
  }
}
