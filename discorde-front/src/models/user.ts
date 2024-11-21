export class User {
  username: string
  chats: string[]


  constructor(username: string, chats: string[]) {
    this.username = username;
    this.chats = chats;
  }
}
