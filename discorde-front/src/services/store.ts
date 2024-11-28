import {User} from '../models/user';
import {createStore, withProps} from '@ngneat/elf';
import {localStorageStrategy, persistState} from '@ngneat/elf-persist-state';

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

export {
  authStore
}
