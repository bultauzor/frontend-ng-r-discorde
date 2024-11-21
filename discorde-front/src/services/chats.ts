import {User} from '../models/user';
import {user$} from './user';
import {base} from './consts';

let latest: User | null = null
user$.subscribe(e => latest = e)

async function createChat(_private: boolean, name: string, members: string[]): Promise<boolean> {
  if (latest == null) {
    alert("Not connected");
    throw "Not connected"
  }

  const res = await fetch(`${base}/chats`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
      "Authorization": `Bearer ${latest.username}`
    },
    body: JSON.stringify({"private": _private, "name": name, "members": members})
  })

  return res.ok
}

async function getChats(): Promise<boolean> {
  if (latest == null) {
    alert("Not connected");
    throw "Not connected"
  }

  const res = await fetch(`${base}/chats`, {
    headers: {
      "Authorization": `Bearer ${latest.username}`
    },
    body: JSON.stringify({"private": _private, "name": name, "members": members})
  })

  return res.ok
}
