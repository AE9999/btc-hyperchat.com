import {Component, OnInit, Output} from '@angular/core';
import {FormBuilder, FormGroup } from "@angular/forms";
import {Observable, ReplaySubject, startWith} from "rxjs";
import {select, Store} from "@ngrx/store";
import {AppState, selectGetStoreWebhookConfigState} from "../../../../store/reducers";
import {IStoreWebhookConfig} from "../../../../model/settings/store-webhook-config/store-webhook-config";


@Component({
  selector: 'app-post-type-input',
  templateUrl: './post-type-input.component.html',
  styleUrls: ['./post-type-input.component.css']
})
export class PostTypeInputComponent implements OnInit {

  formGroup: FormGroup;

  subject: ReplaySubject<{postType: number}> =
    new ReplaySubject<{postType: number}>();

  postOptions = [
    {name: 'ApplicationXWwwFormUrlEncoded', value: 1},
    {name: 'Json', value: 2},
  ]

  defaultPostValue = 1;

  constructor(private formBuilder: FormBuilder,
              private store: Store<AppState>,) {

    this.formGroup = formBuilder.group({
      postType: [this.defaultPostValue]
    });

    this.subject.next({postType:  this.defaultPostValue });

    this.formGroup
        .valueChanges
        .subscribe(x => {
          this.subject.next(x);
        })
  }

  @Output()
  get nextValue() : Observable<{postType: number}> {
    return this.subject;
  }

  ngOnInit(): void {
    let storeWebhookConfig$: Observable<IStoreWebhookConfig | undefined> =
      this.store.pipe(select(selectGetStoreWebhookConfigState),
        select(x => x.storeWebhookConfig));

    storeWebhookConfig$.subscribe((webhookConfig : IStoreWebhookConfig | undefined) => {
      if (webhookConfig != undefined) {
        this.formGroup.patchValue({postType: webhookConfig.postTypeCode});
      }
    });
  }

}
