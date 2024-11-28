import {Component, OnDestroy, OnInit} from '@angular/core';
import {AsyncPipe, NgForOf} from '@angular/common';
import {ActivatedRoute, Router} from '@angular/router';
import {getMessages, liveMessages} from '../../services/chats';
import {Message} from '../../models/message';
import {MessageComponent} from '../../components/message/message.component';
import {Observable} from 'rxjs';
import {User} from '../../models/user';
import {user$} from '../../services/observables';
import {FormsModule} from '@angular/forms';
import {WsCommand} from '../../models/ws-command';

@Component({
  selector: 'app-chat-page',
  imports: [
    NgForOf,
    MessageComponent,
    AsyncPipe,
    FormsModule
  ],
  templateUrl: './chat-page.component.html',
  styleUrl: './chat-page.component.css'
})
export class ChatPageComponent implements OnInit, OnDestroy {
  id: string | null = null
  messages: Message[] = []
  tx: (msg: WsCommand) => void = (_) => {
  }
  rx$: Observable<Message> = new Observable()
  me: User | null = null
  body: string = ""
  private close: () => void = () => {
  };

  constructor(private route: ActivatedRoute, private _router: Router) {
    user$.subscribe(e => {
      if (e == null) _router.navigateByUrl("/")
      this.me = e
    })
  }

  ngOnInit() {
    this.route.paramMap.subscribe(async (params) => {
      this.id = params.get("id")

      this.messages = await getMessages(this.id!);
      const live = await liveMessages(this.id!)
      this.tx = live.tx
      this.rx$ = live.rx$
      this.close = live.close
      this.rx$.subscribe(msg => this.messages.push(msg))
    })
  }

  ngOnDestroy() {
    this.close()
  }

  updateBody($value: string) {
    this.body = $value
  }

  send() {
    const msg = {timestamp: Date.now(), author: this.me?.username || "error", message: this.body}
    this.tx(new WsCommand(this.me?.username || "error", msg))
    this.messages.push(new Message(msg.timestamp, msg.author, msg.message))
    this.body = ""
  }
}
