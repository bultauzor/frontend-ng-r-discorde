import {Component} from '@angular/core';
import {Router} from '@angular/router';
import {getUsers, user$} from '../../services/user';
import {AsyncPipe, NgForOf} from '@angular/common';
import {BehaviorSubject, Observable} from 'rxjs';
import {User} from '../../models/user';

@Component({
  selector: 'app-home-page',
  imports: [
    AsyncPipe,
    NgForOf
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
        getUsers().then(users => {
          this.getUsers$.next(users)
          console.log(users)
        }).catch(err => console.log(err))
      }
    });

  }

  getUsers$ = new BehaviorSubject<User[]>([])
  /*((observer) => {
    getUsers().then(users => {
      observer.next(users)
    })
  });*/
}
