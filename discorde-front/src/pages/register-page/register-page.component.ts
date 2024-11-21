import {Component, Input} from '@angular/core';
import {Router} from '@angular/router';
import {createUser, login, user$} from '../../services/user';
import {FormsModule} from '@angular/forms';

@Component({
  selector: 'app-register-page',
  imports: [
    FormsModule
  ],
  templateUrl: './register-page.component.html',
  styleUrl: './register-page.component.css'
})
export class RegisterPageComponent {
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
    await createUser(this.username, this.password)
    await this._router.navigateByUrl('/login')
  }
}
