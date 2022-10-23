import { Injectable } from '@angular/core';
import {HttpClient} from "@angular/common/http";
import {EndpointService} from "./endpoint.service";
import {Observable} from "rxjs";
import {IProfile} from "../../model/profile/profile";
import {IFindProfileNamesResponse} from "../../model/profile/find-profile-names-response"

@Injectable({
  providedIn: 'root'
})
export class ProfileService {

  constructor(private readonly http: HttpClient,
              private readonly backend: EndpointService) {

  }

  public getGetProfile(username: string): Observable<IProfile> {
    return this.http
               .get<IProfile>(this.backend.getApiBase() + "/profile/get/" + username);
  }

  public getTopProfiles() : Observable<IFindProfileNamesResponse> {
    return this.http
               .get<IFindProfileNamesResponse>(this.backend.getApiBase() + "/profile/top");
  }

  public getFindProfile(username_prefix: string) : Observable<IFindProfileNamesResponse> {
    const url = this.backend.getApiBase() + "/profile/find/" + username_prefix
    return this.http
               .get<IFindProfileNamesResponse>(url);
  }


}
