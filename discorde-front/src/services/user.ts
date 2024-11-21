import {User} from '../models/user';
import {createStore, select, withProps} from '@ngneat/elf';
import {localStorageStrategy, persistState} from '@ngneat/elf-persist-state';
import {base} from './consts';

interface AuthProps {
  user: User | null;
}

const authStore = createStore(
  {name: 'auth'},
  withProps<AuthProps>({user: null})
);

const persist = persistState(authStore, {
  key: 'auth',
  storage: localStorageStrategy,
});

const user$ = authStore.pipe(select((state) => state.user));

let latest: User | null = null
user$.subscribe(e => latest = e)

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
  user$,
  createUser,
  login,
  getUsers,
  getUser
}
