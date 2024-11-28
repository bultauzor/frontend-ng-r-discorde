import {Component} from '@angular/core';
import {user$, users$} from '../../services/observables';
import {AsyncPipe, NgForOf, NgIf} from '@angular/common';
import {UserSelectorComponent} from '../../components/user-selector/user-selector.component';
import {FormsModule} from '@angular/forms';
import {User} from '../../models/user';
import {createChat, getChats} from '../../services/chats';
import {Router} from '@angular/router';

@Component({
  selector: 'app-new-chat-page',
  imports: [
    AsyncPipe,
    NgForOf,
    UserSelectorComponent,
    NgIf,
    FormsModule
  ],
  templateUrl: './new-chat-page.component.html',
  styleUrl: './new-chat-page.component.css'
})
export class NewChatPageComponent {
  name: string = ""
  people: string[] = ["", ""]

  me: User | null = null
  protected readonly users$ = users$;

  constructor(private _router: Router) {
    user$.subscribe(e => {
      if (e == null) _router.navigateByUrl("/")
      this.me = e
    })
    this._router = _router
  }

  delete(guy: string) {
    this.people = this.people.filter(value => value != guy)
  }

  update(old: string, neww: string) {
    for (let i = 0; i < this.people.length; i++) {
      if (this.people[i] == old) {
        this.people[i] = neww
        break
      }
    }
  }

  updateName($value: string) {
    this.name = $value
  }

  async createGroup() {
    if (this.name == "") {
      alert("Invalid name")
      return
    }
    this.people.push(this.me?.username || "")
    this.people = [...new Set(this.people)]
    if (this.people.includes("") || this.people.length < 2) {
      alert("Invalid members")
      return
    }

    await createChat(false, this.name, this.people)
    await getChats()
    await this._router.navigateByUrl("/")
  }

  async privateChat(username: string) {
    await createChat(true, `Chat with ${username}`, [this.me?.username || "", username])
    await getChats()
    await this._router.navigateByUrl("/")
  }
}
