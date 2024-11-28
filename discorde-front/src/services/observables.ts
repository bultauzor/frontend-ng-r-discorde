import {select} from '@ngneat/elf';
import {authStore} from './store';
import {BehaviorSubject} from 'rxjs';
import {User} from '../models/user';
import {Chat} from '../models/chat';

const user$ = authStore.pipe(select((state) => state.user));
const users$ = new BehaviorSubject<User[]>([])
const chats$ = new BehaviorSubject<Chat[]>([])

export {
  user$,
  users$,
  chats$
}
