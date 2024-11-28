import {Component, input, output} from '@angular/core';
import {users$} from "../../services/observables";
import {AsyncPipe, NgForOf} from "@angular/common";
import {ReactiveFormsModule} from '@angular/forms';

@Component({
  selector: 'app-user-selector',
  imports: [
    AsyncPipe,
    NgForOf,
    ReactiveFormsModule
  ],
  templateUrl: './user-selector.component.html',
  styleUrl: './user-selector.component.css'
})
export class UserSelectorComponent {
  value = input("")
  changed = output<string>()
  protected readonly users$ = users$;
}
