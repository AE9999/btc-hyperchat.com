import { Component, OnInit } from '@angular/core';
import {Observable} from "rxjs";
import {TranslateService} from "@ngx-translate/core";
import {KeycloakService} from "keycloak-angular";

@Component({
  selector: 'app-info',
  templateUrl: './home.component.html',
  styleUrls: ['./home.component.css']
})
export class HomeComponent implements OnInit {

  constructor(private translate: TranslateService,
              private keycloakService: KeycloakService,) { }

  ngOnInit(): void {
  }

  async clickRegistration() {
    await this.keycloakService.register();
  }

  get_title(subject: string) : Observable<string> {
    let key = `HOME.${subject}.TITLE`
    return this.translate.get(key);
  }

  get_body(subject: string) : Observable<string> {
    let key = `HOME.${subject}.BODY`
    return this.translate.get(key);
  }

}
