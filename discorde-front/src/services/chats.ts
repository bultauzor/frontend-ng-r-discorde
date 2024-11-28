import {User} from '../models/user';
import {base, wsbase} from './consts';
import {Chat} from '../models/chat';
import {chats$, user$} from './observables';
import {Message} from '../models/message';
import {Observable} from 'rxjs';
import {WsCommand} from '../models/ws-command';

let latest: User | null = null
user$.subscribe((e: User | null) => latest = e)

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

async function getChats(): Promise<void> {
  if (latest == null) {
    throw "Not connected"
  }

  const res = await fetch(`${base}/chats`, {
    headers: {
      "Authorization": `Bearer ${latest.username}`
    }
  })

  if (!res.ok) throw await res.text()

  const chats: Chat[] = (await res.json()).map((e: any) => new Chat(e.id, e.private, e.name, e.members))

  chats$.next(chats)

  setTimeout(getChats, 60000)
}

async function getMessages(id: string): Promise<Message[]> {
  if (latest == null) {
    throw "Not connected"
  }

  const res = await fetch(`${base}/chats/${id}/messages`, {
    headers: {
      "Authorization": `Bearer ${latest.username}`
    }
  })

  if (!res.ok) throw await res.text()

  return (await res.json()).map((e: any) => new Message(e.timestamp, e.author, e.message))
}

async function liveMessages(id: string): Promise<{
  tx: (msg: WsCommand) => void;
  close: () => void;
  rx$: Observable<Message>
}> {
  if (latest == null) {
    throw "Not connected"
  }

  const ws = new WebSocket(`${wsbase}/chats/${id}`, ["realProtocol", latest.username])

  const rx$ = new Observable<Message>(s => {
    ws.onmessage = event => {
      console.log(event)
      console.log(event.data)
      const msg: WsCommand = JSON.parse(event.data)
      s.next(new Message(msg.message.timestamp, msg.message.author, msg.message.message))
    }
  })

  const tx = (msg: WsCommand) => {
    ws.send(JSON.stringify(msg))
  }

  return {tx, rx$, close: () => ws.close()}
}

export {
  createChat,
  getChats,
  getMessages,
  liveMessages
}
