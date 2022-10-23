import { Component, OnInit } from '@angular/core';
import {combineLatest, Observable } from "rxjs";
import {AbstractControl, FormBuilder, FormGroup, ValidationErrors, Validators} from "@angular/forms";
import {IGetProfileState} from "../../store/reducers/profile/get-profile.reducer";
import {Store, select} from "@ngrx/store";
import {IRegisterBtcChatState} from "../../store/reducers/btc-chat/register-btc-chat/register-btc-chat.reducer";
import {IProfile} from "../../model/profile/profile";
import {ActivatedRoute, Router} from "@angular/router";
import {RequestGetProfileAction} from "../../store/actions/profile/get-profile.actions";
import {
  AppState,
  selectBtcPayInvoiceModalState,
  selectGetProfileState,
  selectRegisterBtcChatState
} from "../../store/reducers";
import { RegisterBtcChatRequestAction} from "../../store/actions/btc-chat/register-btc-chat/register-btc-chat.actions";
import {IRegisterBtcChatRequest} from "../../model/btc-chat/register-btc-chat/register-btc-chat-request";
import {amountValidator} from "./validators";
import {MatSlideToggleChange} from "@angular/material/slide-toggle";
import { first } from 'rxjs/operators';
import {environment} from "../../../environments/environment";
import {RegisterBtcChatFormResetService} from "../../service/frontend/register-btc-chat-form-reset.service";



@Component({
  selector: 'app-register-btc-chat',
  templateUrl: './register-btc-chat.component.html',
  styleUrls: ['./register-btc-chat.component.css']
})
export class RegisterBtcChatComponent implements OnInit {

  public profileLoading$: Observable<boolean>;

  public modalLoading$: Observable<boolean>;

  public error$: Observable<boolean>;

  public profile$: Observable<IProfile | undefined>;

  public sendingBtcChat$: Observable<boolean>;

  private registerBtcChatState$: Observable<IRegisterBtcChatState>;

  public formGroup: FormGroup ;

  private routeSub: any;

  private storedSenderName = "";

  constructor(private router: Router,
              private route: ActivatedRoute,
              private formBuilder: FormBuilder,
              private store: Store<AppState>,
              private registerBtcChatFormResetService: RegisterBtcChatFormResetService) {

    let getProfileState: Observable<IGetProfileState>
      = store.pipe(select(selectGetProfileState));

    this.profileLoading$ =
      getProfileState.pipe(select( state => state.loading));
    this.error$ =
      getProfileState.pipe(select( state => state.error));
    this.profile$ =
      getProfileState.pipe(select( state => state.profile));

    this.registerBtcChatState$ = store.pipe(select(selectRegisterBtcChatState));

    this.sendingBtcChat$ = this.registerBtcChatState$
                               .pipe(select( state => state.sending));

    let btcPayInvoiceModalState$ = store.pipe(select(selectBtcPayInvoiceModalState));
    this.modalLoading$ = btcPayInvoiceModalState$.pipe(select( state => state.loading));


    this.formGroup = this.formBuilder.group({
      sender: ['',
              Validators.compose([
                Validators.minLength(environment.registerBtcChatComponent.minNameLength),
                Validators.maxLength(environment.registerBtcChatComponent.maxNameLength)])],
      sendAnonymously: [false],
      amount:  ['', Validators.compose([amountValidator(),
                                                 Validators.required])],
      currency: ['USD', Validators.compose([Validators.required])],
      message: ['', Validators.maxLength(environment.registerBtcChatComponent.maxMessageLength)],
    }, { validator: this.ensureSender } );

    this.registerBtcChatFormResetService
        .resetBtcChatForm
        .subscribe(x => {
          this.formGroup.reset();
        })
  }

  public defaultCurrency(profile: IProfile | null | undefined) : string {
    if (!profile || !profile.currencies) {
      return "Nan"
    }
    return profile.currencies[0]
  }

  get remainingText() : number {
    const exists: boolean = (this.formGroup.get('message')?.value);
    const textLength: number = exists ? (this.formGroup.get('message')?.value.length)
                                      :  0;
    return 200 - textLength;
  }

  get hasSenderError() : boolean {
    const errors = this.formGroup.get('sender')?.touched
                   && this.formGroup.errors;
    if (!errors) {
      return false;
    }
    return errors['noSender']
  }

  public toggleSendAnonymously(event_: MatSlideToggleChange): void {
    const { checked } = event_;
    if (checked) {
      this.formGroup.get('sender')?.disable();
      this.storedSenderName = this.formGroup.get('sender')?.value;
      this.formGroup.get('sender')?.reset();
    } else {
      this.formGroup.get('sender')?.enable();
      this.formGroup.get('sender')?.setValue(this.storedSenderName);
    }
  }

  ngOnInit(): void {
    this.routeSub = this.route.params.subscribe(params => {

      if (params["profileName"]) {
        let profileName :string  = params["profileName"];

        const action = RequestGetProfileAction( { payload: { profileName: profileName} } );

        this.store.dispatch(action)
      } else {
        this.router.navigate(['/']);
      }
    });
  }

  ensureSender(control: AbstractControl): ValidationErrors | null {

    const sender = control.get("sender")?.value;
    const sendAnonymously = control.get("sendAnonymously")?.value;

    if (sendAnonymously) {
      return null
    }

    return sender ? null : { 'noSender': true }

  }

  tip(): void {


    if (!this.formGroup.errors) {

      combineLatest([this.registerBtcChatState$, this.profile$])
        .pipe(first()).subscribe(([registerBtcPayState, profile]) =>  {

        if (registerBtcPayState?.sending) {
          return;
        }

        if (!profile) {
          return;
        }

        const registerBtcChatRequest : IRegisterBtcChatRequest = {
          storeId: profile.storeId,
          currency: this.formGroup.get('currency')?.value, // T
          amount: parseInt(this.formGroup.get('amount')?.value),
          message: this.formGroup.get('message')?.value,
          sender: this.formGroup.get('sendAnonymously')?.value ? undefined
                                                                        : this.formGroup.get('sender')?.value,
        }

        this.store
            .dispatch(RegisterBtcChatRequestAction({payload: { registerBtcChatRequest: registerBtcChatRequest }}))
      });

    }
  }

  ngOnDestroy() :void {
    this.routeSub.unsubscribe();
  }
}
