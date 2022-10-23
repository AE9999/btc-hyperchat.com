import {Component, Input, OnInit, Output} from '@angular/core';
import {AbstractControl, FormArray, FormBuilder, FormGroup, Validators} from "@angular/forms";
import {combineLatest, distinctUntilChanged, filter, Observable, ReplaySubject, startWith, Subscription} from "rxjs";
import {select, Store} from "@ngrx/store";
import {AppState, selectGetStoreWebhookConfigState} from "../../../../store/reducers";
import {IStoreWebhookConfig} from "../../../../model/settings/store-webhook-config/store-webhook-config";
import { isEqual } from "lodash";


@Component({
  selector: 'app-dynamic-list',
  templateUrl: './dynamic-list.component.html',
  styleUrls: ['./dynamic-list.component.css']
})
export class DynamicListComponent implements OnInit {

  @Input()
  titleKey: string = '';

  @Input()
  explanationKey: string = '';

  @Input()
  namePlaceholderKey: string = '';

  @Input()
  valuePlaceholderKey: string = '';

  @Input()
  noAttributeKey: string = '';

  @Input()
  addKey: string = '';

  @Input()
  configAttribute: string |undefined;

  formGroup: FormGroup;

  subject: ReplaySubject<{ name: string,  value: string }[]> =
    new ReplaySubject<{name: string; value: string}[]>();

  subscription: Subscription | undefined = undefined;


  // Based on https://stackblitz.com/edit/dynamic-angular-form-part-1-final-sfoma9?file=src%2Fapp%2Fapp.component.ts

  // https://therichpost.com/angular-dynamically-add-and-remove-form-fields-using-formbuilder-formgroup-formarray/

  constructor(private formBuilder: FormBuilder,
              private store: Store<AppState>,) {
    this.formGroup = formBuilder.group({
      values: this.formBuilder.array([])
    });

    this.subject.next([]);
  }

  @Output()
  get nextValue() : Observable<{ name: string,  value: string }[]> {
      return this.subject.asObservable();
  }

  ngOnInit(): void {

    let storeWebhookConfig$: Observable<IStoreWebhookConfig> =
      this.store.pipe(select(selectGetStoreWebhookConfigState),
                      select(x => x.storeWebhookConfig),
                      filter(x => x != undefined)) as Observable<IStoreWebhookConfig>;

    storeWebhookConfig$ =
      storeWebhookConfig$.pipe(distinctUntilChanged((prev, curr) => isEqual(prev, curr)))

    storeWebhookConfig$.subscribe((webhookConfig: IStoreWebhookConfig) => {

      if (webhookConfig != undefined
        && this.configAttribute != undefined) {
        // https://bobbyhadz.com/blog/typescript-access-object-property-dynamically

        type ObjectKey = keyof typeof webhookConfig;

        const myVar = this.configAttribute as ObjectKey;

        const record: Record<string, string> = webhookConfig[myVar] as Record<string, string>;

        const nextValues:  {name: string; value: string}[] = this.convert(record);

        this.formGroup = this.formBuilder.group({
          values: this.formBuilder.array([])
        });

        for (let i = 0; i < nextValues.length; i++ ) {
          const nextValue = nextValues[i];
          this.addEntry(nextValue.name, nextValue.value);
        }
        this.combineObservables();
      }
    });
  }

  convert(record: Record<string, string>) : {name: string; value: string}[] {
    const rvalue : {name: string; value: string}[] = []


    if (record) {

      Object.entries(record).forEach(entry => {
        rvalue.push({name: entry[0], value: entry[1]})
      })
    }

    return rvalue;
  }

  get values() {
    return this.formGroup.controls["values"] as FormArray;
  }

  get noAttributes() {
    return this.values.length == 0;
  }


  combineObservables() {
    if (this.subscription) {
      this.subscription.unsubscribe();
      this.subscription = undefined;
    }

    let observables: Observable<any>[] = []
    for (let i = 0; i < this.values.length; i++) {
      let observable = this.values.at(i)?.valueChanges
                                         .pipe(startWith(null));
      if (observable){
        observables.push(observable)
      }
    }

    let me = this;
    this.subscription =
      combineLatest(observables)
        .subscribe(next => {
          let nextValues = [];
          for (let i = 0; i < me.values.controls.length; i++) {
            nextValues.push(me.values.controls[i].value)
          }
          me.subject.next(nextValues);
      });
  }

  addEntry(name: string, value: string) {
    const entry = this.formBuilder.group({
      name: [name, Validators.required],
      value: [value]
    });
    this.values.push(entry);
    this.combineObservables();
  }

  getFormGroup(control: AbstractControl) { return control as FormGroup; }


  deleteEntry(index: number) {
    this.values.removeAt(index);
    this.combineObservables();
  }

  get hasError() : boolean {
    return (this.formGroup) && this.formGroup.invalid;
  }

}
