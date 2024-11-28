import {BehaviorSubject} from 'rxjs';

export class Message {
  timestamp: Date
  author: string
  message: string
  timestamp$ = new BehaviorSubject<string>("now")
  message$


  constructor(timestamp: number, author: string, message: string) {
    this.timestamp = new Date(timestamp);
    this.author = author;
    this.message = message;
    this.message$ = new BehaviorSubject<string>(message)
    setInterval(() => this.timestamp$.next(this.formatTimestamp()), 30)
    this.parseMessage()
  }

  private formatTimestamp() {
    const diff = Math.round(((new Date()).getTime() - this.timestamp.getTime()) / 1000)
    if (diff > 60 * 60 * 24) {
      return `${Math.round(diff / 60 * 60 * 24)} days ago`
    } else if (diff == 60 * 60 * 24) {
      return `1 day ago`
    } else if (diff > 60 * 60) {
      return `${Math.round(diff / 60 * 60)} hours ago`
    } else if (diff == 60 * 60) {
      return `1 hour ago`
    } else if (diff > 60) {
      return `${Math.round(diff / 60)} minutes ago`
    } else if (diff == 60) {
      return `1 minute ago`
    } else {
      return "now"
    }
  }

  private parseMessage() {
    if (this.message.startsWith("/")) {
      switch (this.message.substring(1).split(" ")[0]) {
        case "tableflip":
          this.message$.next("(╯°□°)╯︵ ┻━┻")
          break
        case "type":
          const msg = this.message.substring(6)
          for (let i = 0; i <= msg.length; i++) {
            setTimeout(() => {
              this.message$.next(msg.substring(0, i))
            }, 200 * (i + 1))
          }
      }
    }
  }
}
