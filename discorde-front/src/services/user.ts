import {User} from '../models/user';
import {base} from './consts';
import {user$, users$} from './observables';
import {authStore} from './store';
import {getChats} from './chats';


let latest: User | null = null
user$.subscribe(async e => {
  latest = e

  if (e != null) {
    users$.next(await getUsers())
    await getChats()
  }
})

async function createUser(username: string, password: string): Promise<boolean> {
  const res = await fetch(`${base}/users`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({"username": username, "password": password})
  })

  return res.ok
}

async function login(username: string, password: string) {
  const res = await fetch(`${base}/login`, {
    method: "POST",
    headers: {
      "Content-Type": "application/json"
    },
    body: JSON.stringify({"username": username, "password": password})
  });

  if (!res.ok) {
    alert("Invalid login");
    return
  }

  const user: User = (await res.json()).user

  authStore.update((state) => ({
    ...state,
    user: user,
  }));
}

async function logout() {
  authStore.update((state) => ({
    ...state,
    user: null,
  }));
}


async function getUsers(): Promise<User[]> {

  if (latest == null) {
    alert("Not connected");
    throw "Not connected"
  }

  const res = await fetch(`${base}/users`, {
    headers: {
      "Authorization": `Bearer ${latest.username}`
    }
  })

  return await res.json();
}

async function getUser(id: string): Promise<User> {
  if (latest == null) {
    alert("Not connected");
    throw "Not connected"
  }

  const res = await fetch(`${base}/users/${id}`, {
    headers: {
      "Authorization": `Bearer ${latest.username}`
    }
  })

  return await res.json();
}


export {
  authStore,
  createUser,
  login,
  logout,
  getUser
}
