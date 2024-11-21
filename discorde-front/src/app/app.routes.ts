import { Routes } from '@angular/router';
import {LoginPageComponent} from '../pages/login-page/login-page.component';
import {HomePageComponent} from '../pages/home-page/home-page.component';
import {RegisterPageComponent} from '../pages/register-page/register-page.component';
import {ChatPageComponent} from '../pages/chat-page/chat-page.component';

export const routes: Routes = [
  {path: "", component: HomePageComponent},
  {path: "login", component: LoginPageComponent},
  {path: "register", component: RegisterPageComponent},
  {path: "chat", component: ChatPageComponent},
];
