import { Component, OnInit } from '@angular/core';
import {combineLatest, Observable, of} from "rxjs";
import {AbstractControl, FormBuilder, FormGroup, ValidationErrors, Validators} from "@angular/forms";
import {select, Store} from "@ngrx/store";
import {
} from "../../../store/reducers";
import {TranslateService} from "@ngx-translate/core";


@Component({
  selector: 'app-settings',
  templateUrl: './settings.component.html',
  styleUrls: ['./settings.component.css']
})
export class SettingsComponent implements OnInit {

  tooltip_configuration:  string = '';

  tooltip_activation: string = '';

  tooltip_test: string = '';

  tooltip_btc_pay: string = '';

  tooltip_account: string = '';

  constructor(private translate: TranslateService) {
  }

  ngOnInit(): void {

    this.translate.get('SETTINGS.CONFIGURATION_TAB_TITLE_TOOLTIP')
      .subscribe(value => this.tooltip_configuration = value);

    this.translate.get('SETTINGS.ACTIVATION_TAB_TITLE_TOOLTIP')
      .subscribe(value => this.tooltip_activation = value);

    this.translate.get('SETTINGS.TEST_TAB_TITLE_TOOLTIP')
      .subscribe(value => this.tooltip_test = value);

    this.translate.get('SETTINGS.BTC_PAY_TITLE_TOOLTIP')
      .subscribe(value => this.tooltip_btc_pay = value);

    this.translate.get('SETTINGS.ACCOUNT_TITLE_TOOLTIP')
      .subscribe(value => this.tooltip_account = value);

  }


}
