import { NgModule } from '@angular/core';
import { RouterModule, Routes } from '@angular/router';
import {SettingsComponent} from "./components/settings/settings/settings.component";
import {MyReceivedBtcChats} from "./components/my-received-btc-chats/my-received-btc-chats/my-received-btc-chats";
import {RegisterBtcChatComponent} from "./components/register-btc-chat/register-btc-chat.component";
import {HomeComponent} from "./components/home/home.component";
import {AuthGuard} from "./guard/auth.guard";
import {FindProfileComponent} from "./components/find-profile/find-profile/find-profile.component";
import {TermsOfServiceComponent} from "./components/terms-of-service/terms-of-service.component";
import {PrivacyPolicyComponent} from "./components/privacy-policy/privacy-policy.component";

const routes: Routes = [
  {path: '', component: HomeComponent, pathMatch: 'full'},
  {path: 'home', component: HomeComponent},
  {path: 'tip', component: FindProfileComponent, pathMatch: 'full' },
  {path: 'tip/:profileName', component: RegisterBtcChatComponent, pathMatch: 'full'},
  {path: 'read', component: MyReceivedBtcChats,  canActivate: [AuthGuard] },
  {path: 'settings', component: SettingsComponent, canActivate: [AuthGuard]},
  {path: 'tos', component: TermsOfServiceComponent },
  {path: 'privacy', component: PrivacyPolicyComponent },
];

@NgModule({
  imports: [RouterModule.forRoot(routes, { useHash: true})],
  exports: [RouterModule]
})
export class AppRoutingModule { }
