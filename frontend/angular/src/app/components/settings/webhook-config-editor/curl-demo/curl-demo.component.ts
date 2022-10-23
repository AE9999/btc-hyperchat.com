import {Component, Input, OnInit, Output} from '@angular/core';
import {Observable, of, ReplaySubject} from "rxjs";
import {PostType} from "../../../../model/settings/store-webhook-config/post-type";
import {IMyReceivedBtcChat} from "../../../../model/btc-chat/my-received-btc-chats/my-received-btc-chat";
import {TranslateService} from "@ngx-translate/core";


declare var Mustache : any;

@Component({
  selector: 'app-curl-demo',
  templateUrl: './curl-demo.component.html',
  styleUrls: ['./curl-demo.component.css']
})
export class CurlDemoComponent implements OnInit {

  exampleUrlCommand$: ReplaySubject<string> = new ReplaySubject<string>();

  @Input()
  currentCommand$: Observable<[{ webhookUrl: string; },
                               { postType: number; },
                               { name: string; value: string; }[],
                               { name: string; value: string; }[],
                               { name: string; value: string; }[],
                               IMyReceivedBtcChat | undefined]> | undefined;
  constructor(private translateService: TranslateService) {

  }

  ngOnInit() {
    if (!this.currentCommand$) {
      console.log("Expected current command to be set!")
      return;
    }

    this.currentCommand$
      .subscribe(([urlInput,
                    postTypeInput,
                    headerInput,
                    bodyInput,
                    queryInput,
                    exampleBtcChat]) =>  {

        this.calculateCommand(urlInput,
          postTypeInput,
          headerInput,
          bodyInput,
          queryInput,
          exampleBtcChat).subscribe(command =>
          this.exampleUrlCommand$
            .next(command)
        );}
      );
  }

  private calculateCommand(urlInput: { webhookUrl : string},
                           postTypeInput: {postType: number},
                           headerInput: { name: string; value: string }[] | null,
                           bodyInput:  { name: string; value: string }[] | null,
                           queryInput: { name: string; value: string }[] | null,
                           myReceivedBtcChat: IMyReceivedBtcChat | undefined) : Observable<string> {

    if (!urlInput.webhookUrl) {
      return this.translateService.get("SETTINGS.CURL_COMMAND_PLACE_HOLDER");
    }

    let dataCommand = "";
    if (postTypeInput.postType == PostType.ApplicationXWwwFormUrlEncoded) {
      dataCommand = this.transformToApplicationXWwwFormUrlEncodedBody(bodyInput);
    } else if (postTypeInput.postType == PostType.Json) {
      dataCommand = this.transformToJSONBody(bodyInput);
    } else {
      return this.translateService.get("SETTINGS.CURL_COMMAND_PLACE_HOLDER");
    }

    let host = this.getHost(urlInput, queryInput);

    let source = `${this.getBaseCommand()} ${this.transformHeaders(headerInput)} ${dataCommand} ${host}`

    if (!myReceivedBtcChat) {
      return of(source);
    }

    try {
      return of(Mustache.render(source, myReceivedBtcChat));
    } catch (e) {
      return of(source);
    }
  }

  private getBaseCommand() : string {
    return "curl -X POST "
  }

  private getHost(urlInput: { webhookUrl : string} ,
                  queryInput: { name: string; value: string }[] | null) : string {

    const host = urlInput ? urlInput.webhookUrl : "";

    return `${host}${this.queryString(queryInput)}`
  }

  private queryString(queryInput: { name: string; value: string }[] | null) : string {
    if (!queryInput || queryInput.length == 0) {
      return "";
    }
    let rvalue = "?"
    for (let i = 0; i < queryInput.length; i++) {
      let input = queryInput[i];
      let seperator = i == 0 ? "": "&";
      rvalue += `${seperator}${input.name}=${input.value}`
    }
    return rvalue;
  }

  private transformHeaders(headers: { name: string; value: string }[] | null): string {
    if (!headers) {
      return "";
    }
    let rvalue = ""
    for (let i = 0; i < headers.length; i++) {
      rvalue += this.transformToHeader(headers[i]);
    }
    return rvalue;
  }

  private transformToHeader(header: { name: string; value: string }) : string {
    return ` -H '${header.name}: ${header.value}'`
  }

  private transformToApplicationXWwwFormUrlEncodedBody(bodyValues: { name: string; value: string }[] | null) : string {
    if (!bodyValues) {
      return " -H 'Content-Type: application/x-www-form-urlencoded'"
    }

    let dataArgument = ""
    for (let i = 0; i < bodyValues.length; i++) {
      let input = bodyValues[i];
      let seperator = i == 0 ? "": "&";
      dataArgument += `${seperator}${input.name}=${input.value}`
    }

    return ` -H 'Content-Type: application/x-www-form-urlencoded' -d '${dataArgument}'`;
  }

  private transformToJSONBody(bodyValues: { name: string; value: string }[] | null) : string {
      if (!bodyValues) {
        return " -H 'Content-Type: application/json' -d '{}'"
      }
      let rvalue: any = { };
      for (let i = 0; i < bodyValues.length; i++) {
        let bodyValue = bodyValues[i];
        rvalue[bodyValue.name] = bodyValue.value;
      }

      return ` -H 'Content-Type: application/json' '${JSON.stringify(rvalue)}'`;
  }

}
