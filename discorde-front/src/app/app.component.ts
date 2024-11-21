import { Component } from '@angular/core';
import {RouterLink, RouterOutlet} from '@angular/router';
import {user$} from '../services/user';
import {AsyncPipe, NgIf} from '@angular/common';
import {map} from 'rxjs';

@Component({
  selector: 'app-root',
  imports: [RouterOutlet, RouterLink, AsyncPipe, NgIf],
  templateUrl: './app.component.html',
  styleUrl: './app.component.css'
})
export class AppComponent {
  title = 'discorde-front';
  connected$ = user$.pipe(map(e => e != null));

}
