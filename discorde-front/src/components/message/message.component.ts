import {Component, input} from '@angular/core';

@Component({
  selector: 'app-message',
  imports: [],
  templateUrl: './message.component.html',
  styleUrl: './message.component.css'
})
export class MessageComponent {
  author = input("")
  timestamp = input("")
  message = input("")
}