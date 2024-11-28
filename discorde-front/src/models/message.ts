import {BehaviorSubject} from 'rxjs';

export class Message {
  timestamp: Date
  author: string
  message: string
  timestamp$ = new BehaviorSubject<string>("now")


  constructor(timestamp: number, author: string, message: string) {
    this.timestamp = new Date(timestamp);
    this.author = author;
    this.message = message;
    setInterval(() => this.timestamp$.next(this.formatTimestamp()), 30)
  }

  private formatTimestamp() {
    const diff  = Math.round(((new Date()).getTime() - this.timestamp.getTime()) / 1000)
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
}
