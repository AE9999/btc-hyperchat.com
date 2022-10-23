import {APP_INITIALIZER, NgModule} from '@angular/core';
import { BrowserModule } from '@angular/platform-browser';
import {HttpClient, HttpClientModule} from '@angular/common/http';
import { AppRoutingModule } from './app-routing.module';
import { AppComponent } from './app.component';
import { BrowserAnimationsModule } from '@angular/platform-browser/animations';
import { RegisterBtcChatComponent } from './components/register-btc-chat/register-btc-chat.component';
import { MyReceivedBtcChats } from './components/my-received-btc-chats/my-received-btc-chats/my-received-btc-chats';
import { SettingsComponent } from './components/settings/settings/settings.component';
import { StoreModule } from '@ngrx/store';
import { EffectsModule } from '@ngrx/effects';
import { RegisterBtcChatEffects } from "./store/effects/btc-chat/register-btc-chat/register-btc-chat.effects";
import { BtcPayInvoiceModalEffects } from "./store/effects/btc-chat/register-btc-chat/bty-pay-invoice-modal.effects";
import { reducers } from './store/reducers';
import {MatSnackBarModule} from "@angular/material/snack-bar";
import {FormsModule, ReactiveFormsModule} from "@angular/forms";
import {MatButtonModule} from "@angular/material/button";
import {MatMenuModule} from "@angular/material/menu";
import {MatCardModule} from "@angular/material/card";
import {MatToolbarModule} from "@angular/material/toolbar";
import {MatFormFieldModule} from "@angular/material/form-field";
import {MatGridListModule} from "@angular/material/grid-list";
import {MatIconModule} from "@angular/material/icon";
import {MatSidenavModule} from "@angular/material/sidenav";
import {MatSelectModule} from "@angular/material/select";
import {MatListModule} from "@angular/material/list";
import {MatProgressSpinnerModule} from "@angular/material/progress-spinner";
import {MatTabsModule} from "@angular/material/tabs";
import {MatDatepickerModule} from "@angular/material/datepicker";
import {MatExpansionModule} from "@angular/material/expansion";
import {MatSlideToggleModule} from "@angular/material/slide-toggle";
import {MatCheckboxModule} from "@angular/material/checkbox";
import {MatProgressBarModule} from "@angular/material/progress-bar";
import {MatStepperModule} from "@angular/material/stepper";
import {TranslateLoader, TranslateModule} from "@ngx-translate/core";
import {MatInputModule} from "@angular/material/input";
import {MatOptionModule} from "@angular/material/core";
import {MatChipsModule} from "@angular/material/chips";
import {TranslateHttpLoader} from "@ngx-translate/http-loader";
import {ScrollingModule} from "@angular/cdk/scrolling";
import { HomeComponent } from './components/home/home.component';
import { StoreDevtoolsModule } from '@ngrx/store-devtools';
import { environment } from '../environments/environment';
import {GetProfileEffects} from "./store/effects/profile/get-profile.effects";
import {initializeKeycloak} from "./init/keycloak-init.factory";
import {KeycloakAngularModule, KeycloakService} from "keycloak-angular";
import { MyReceivedBtcChatComponent } from './components/my-received-btc-chats/my-received-btc-chat/my-received-btc-chat.component';
import { FindProfileComponent } from './components/find-profile/find-profile/find-profile.component';
import { SearchProfileAutocompleteComponent } from './components/find-profile/search-profile-autocomplete/search-profile-autocomplete.component';
import { MatAutocompleteModule } from '@angular/material/autocomplete';
import {FindProfilesEffects} from "./store/effects/profile/find-profiles.effects";
import {TopProfilesEffects} from "./store/effects/profile/top-profiles.effect";
import { TopProfilesComponent } from './components/find-profile/top-profiles/top-profiles.component';
import { FlexLayoutModule } from "@angular/flex-layout";
import {TestWebhookEffects} from "./store/effects/settings/webhook/test-webhook.effect";

import {GetTestBtcChatEffects} from "./store/effects/settings/webhook/get-test-btc-chat.effect";
import { CurlDemoComponent } from './components/settings/webhook-config-editor/curl-demo/curl-demo.component';
import { WebhookTestComponent } from './components/settings/webhook-test/webhook-test.component';
import { WebhookConfigEditorComponent } from './components/settings/webhook-config-editor/webhook-config-editor/webhook-config-editor.component';
import { WebhookActivationEditorComponent } from './components/settings/webhook-activation-editor/webhook-activation-editor.component';
import { DynamicListComponent } from './components/settings/webhook-config-editor/dynamic-list/dynamic-list.component';
import {
  GetStoreWebhookActivationEffects
} from "./store/effects/settings/webhook/store-webhook-activation/get-store-webhook-activation.effect";
import {
  UpdateStoreWebhookActivationEffects
} from "./store/effects/settings/webhook/store-webhook-activation/update-store-webhook-activation.effect";
import {
  GetStoreWebhookConfigEffects
} from "./store/effects/settings/webhook/store-webhook-config/get-store-webhook-config.effect";
import {
  UpdateStoreWebhookConfigEffects
} from "./store/effects/settings/webhook/store-webhook-config/update-store-webhook-config.effect";
import {MatRadioModule} from "@angular/material/radio";
import { UrlInputComponent } from './components/settings/webhook-config-editor/url-input/url-input.component';
import { PostTypeInputComponent } from './components/settings/webhook-config-editor/post-type-input/post-type-input.component';
import {MatTooltipModule} from "@angular/material/tooltip";
import { BtcPaySettingsComponent } from './components/settings/btc-pay-settings/btc-pay-settings.component';
import {DeleteAccountEffects} from "./store/effects/settings/delete-account.effect";
import {BtcPayConfigEffects} from "./store/effects/settings/btc-pay/btc-pay-config.effect";
import {ResetBtcPayPasswordEffect} from "./store/effects/settings/btc-pay/reset-btc-pay-password.effect";
import { AccountSettingsComponent } from './components/settings/account-settings/account-settings/account-settings.component';
import { PrivacyPolicyComponent } from './components/privacy-policy/privacy-policy.component';
import { TermsOfServiceComponent } from './components/terms-of-service/terms-of-service.component';
import { ConfirmAccountDeletionComponent } from './components/settings/account-settings/confirm-account-deletion/confirm-account-deletion.component';
import {MatDialogModule} from "@angular/material/dialog";
import {ProcessBtcChatEffects} from "./store/effects/btc-chat/my-received-btc-chats/process-btc-chat.effects";
import {
  GetMyReceivedBtcChatsEffects
} from "./store/effects/btc-chat/my-received-btc-chats/get-my-received-btc-chats.effects";


@NgModule({
  declarations: [
    AppComponent,
    RegisterBtcChatComponent,
    MyReceivedBtcChats,
    SettingsComponent,
    HomeComponent,
    MyReceivedBtcChatComponent,
    FindProfileComponent,
    SearchProfileAutocompleteComponent,
    TopProfilesComponent,
    CurlDemoComponent,
    WebhookTestComponent,
    WebhookConfigEditorComponent,
    WebhookActivationEditorComponent,
    DynamicListComponent,
    UrlInputComponent,
    PostTypeInputComponent,
    BtcPaySettingsComponent,
    AccountSettingsComponent,
    PrivacyPolicyComponent,
    TermsOfServiceComponent,
    ConfirmAccountDeletionComponent,
  ],
  imports: [
    BrowserModule,
    AppRoutingModule,
    BrowserAnimationsModule,
    FormsModule,
    KeycloakAngularModule,
    MatSnackBarModule,
    HttpClientModule,
    MatAutocompleteModule,
    MatSnackBarModule,
    MatButtonModule,
    MatMenuModule,
    MatToolbarModule,
    MatSnackBarModule,
    MatDialogModule,
    MatCardModule,
    MatInputModule,
    MatToolbarModule,
    MatFormFieldModule,
    MatGridListModule,
    MatIconModule,
    MatSidenavModule,
    MatSelectModule,
    MatOptionModule,
    MatListModule,
    MatProgressSpinnerModule,
    MatTabsModule,
    MatDatepickerModule,
    MatExpansionModule,
    MatSlideToggleModule,
    MatCheckboxModule,
    MatChipsModule,
    MatProgressBarModule,
    MatStepperModule,
    FlexLayoutModule,
    ReactiveFormsModule,

    TranslateModule.forRoot({
      loader: {
        provide: TranslateLoader,
        useFactory: httpTranslateLoader,
        deps: [HttpClient]
      }
    }),
    ScrollingModule,
    StoreModule.forRoot(reducers),
    EffectsModule.forRoot(
      [ProcessBtcChatEffects,
                GetMyReceivedBtcChatsEffects,
                RegisterBtcChatEffects,
                BtcPayInvoiceModalEffects,
                GetProfileEffects,
                GetStoreWebhookActivationEffects,
                UpdateStoreWebhookActivationEffects,
                GetStoreWebhookConfigEffects,
                UpdateStoreWebhookConfigEffects,
                TestWebhookEffects,
                FindProfilesEffects,
                GetTestBtcChatEffects,
                TopProfilesEffects,
                DeleteAccountEffects,
                BtcPayConfigEffects,
                ResetBtcPayPasswordEffect]),
    StoreDevtoolsModule.instrument({maxAge: 25, logOnly: environment.production}),
    MatRadioModule,
    MatTooltipModule
  ],
  providers: [
    {
      provide: APP_INITIALIZER,
      useFactory: initializeKeycloak,
      multi: true,
      deps: [KeycloakService],
    }
  ],
  bootstrap: [AppComponent]
})
export class AppModule { }

export function httpTranslateLoader(http: HttpClient) {
  return new TranslateHttpLoader(http);
}

