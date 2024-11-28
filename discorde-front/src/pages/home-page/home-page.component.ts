import {Component} from '@angular/core';
import {Router, RouterLink} from '@angular/router';
import {AsyncPipe, NgForOf} from '@angular/common';
import {BehaviorSubject, Observable} from 'rxjs';
import {User} from '../../models/user';
import {chats$, user$, users$} from '../../services/observables';
import {MessageComponent} from '../../components/message/message.component';

@Component({
  selector: 'app-home-page',
  imports: [
    AsyncPipe,
    NgForOf,
    RouterLink,
    MessageComponent
  ],
  templateUrl: './home-page.component.html',
  styleUrl: './home-page.component.css'
})
export class HomePageComponent {
  constructor(private _router: Router) {

    user$.subscribe(value => {
      if (value == null) _router.navigateByUrl("/login")
      else {
        console.log("else")
      }
    });

  }
  /*((observer) => {
    getUsers().then(users => {
      observer.next(users)
    })
  });*/
  protected readonly users$ = users$;
  protected readonly chats$ = chats$;
}
