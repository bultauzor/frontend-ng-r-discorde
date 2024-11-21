import {Component, Input} from '@angular/core';
import {FormsModule} from '@angular/forms';
import {login, user$} from '../../services/user';
import {Router, RouterLink} from '@angular/router';

@Component({
  selector: 'app-login-page',
  imports: [
    FormsModule,
    RouterLink
  ],
  templateUrl: './login-page.component.html',
  styleUrl: './login-page.component.css'
})
export class LoginPageComponent {
  @Input() username: string = ""
  @Input() password: string = ""

  constructor(private _router: Router) {

    user$.subscribe(value => {
      if (value != null) _router.navigateByUrl("/")
    });
  }

  updateUsername($value: string) {
    this.username = $value
  }

  updatePassword($value: string) {
    this.password = $value
  }

  async send() {
    await login(this.username, this.password)
    await this._router.navigateByUrl('/')
  }
}
