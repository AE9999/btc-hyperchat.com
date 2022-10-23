import { Component, OnInit, Output } from '@angular/core';
import {FormBuilder, FormGroup, Validators} from "@angular/forms";
import {correctWebhookUrl} from "../validator";
import {Observable, ReplaySubject, startWith} from "rxjs";
import {select, Store} from "@ngrx/store";
import {
  AppState,
  selectGetStoreWebhookActivationState,
  selectGetStoreWebhookConfigState
} from "../../../../store/reducers";
import {
  IGetStoreWebhookConfigState
} from "../../../../store/reducers/settings/webhook/store-webhook-config/get-store-webhook-config.reducer";
import {IStoreWebhookConfig} from "../../../../model/settings/store-webhook-config/store-webhook-config";

@Component({
  selector: 'app-url-input',
  templateUrl: './url-input.component.html',
  styleUrls: ['./url-input.component.css']
})
export class UrlInputComponent implements OnInit {


  formGroup: FormGroup;

  subject: ReplaySubject<{webhookUrl: string}> =
    new ReplaySubject<{webhookUrl: string}>();


  constructor(private formBuilder: FormBuilder,
              private store: Store<AppState>,) {

    this.formGroup = formBuilder.group({
      webhookUrl: ['', Validators.compose([correctWebhookUrl()])]
    });

    this.store.pipe(select(selectGetStoreWebhookActivationState))

    this.subject.next({webhookUrl:  '' });

    this.formGroup
        .valueChanges
        .subscribe(x => {
          this.subject.next(x);
        });
  }

  @Output()
  get nextValue() : Observable<{webhookUrl: string}> {
    return this.subject;
  }

  get hasError() : boolean {
    return (this.formGroup)
           && this.formGroup.invalid
           && this.formGroup.controls['webhookUrl'].touched;
  }

  ngOnInit(): void {
    let storeWebhookConfig$: Observable<IStoreWebhookConfig | undefined> =
      this.store.pipe(select(selectGetStoreWebhookConfigState),
        select(x => x.storeWebhookConfig));

    storeWebhookConfig$.subscribe((webhookConfig : IStoreWebhookConfig | undefined) => {
      if (webhookConfig != undefined) {
        this.formGroup.patchValue({webhookUrl: webhookConfig.url});
      }
    });
  }

}
