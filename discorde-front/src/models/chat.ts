export class Chat {
  id: string
  _private: boolean
  name: string
  members: string[]


  constructor(id: string, _private: boolean, name: string, members: string[]) {
    this.id = id;
    this._private = _private;
    this.name = name;
    this.members = members;
  }
}
